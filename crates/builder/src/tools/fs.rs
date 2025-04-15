use crate::project::ActionResult;
use crate::project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct ReadFile {
    pub file_path: String,
}

impl ReadFile {
    pub fn signature(&self) -> String {
        format!("read_file: {}", self.file_path)
    }
    pub fn exec(&self, project: Project) -> ActionResult {
        let project_dir = &project.dir;
        let file_path = project_dir.join(&project.cwd).join(&self.file_path);
        if !file_path.exists() {
            return project.add_error(&self.signature(), "File not found");
        }
        if !project.path_is_allowed(&file_path) {
            return project.add_error(&self.signature(), "Access denied to file");
        }
        match std::fs::read_to_string(file_path) {
            Ok(content) => project.add_success(&self.signature(), "File content read successfully"),
            Err(_) => project.add_error(&self.signature(), "Error reading file - file not found"),
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct WriteFile {
    pub file_path: String,
    pub content: String,
}

impl WriteFile {
    pub fn signature(&self) -> String {
        format!("write_file: {}", self.file_path)
    }
    pub fn exec(&self, project: Project) -> ActionResult {
        let project_dir = &project.dir;
        let file_path = project_dir.join(&project.cwd).join(&self.file_path);

        if !project.path_is_allowed(&file_path) {
            return project.add_error(
                &self.signature(),
                "Access denied to file - file outside root dir",
            );
        }
        let parent_dir = file_path.parent().unwrap();
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir).unwrap();
        }
        if !file_path.exists() {
            std::fs::File::create(&file_path).unwrap();
        }
        match std::fs::write(&file_path, &self.content) {
            Ok(_) => project.add_success(&self.signature(), "File content written successfully"),
            Err(_) => project.add_error(&self.signature(), "Error writing file - file not found"),
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct ApplyPatchToFile {
    pub file_path: String,
    #[schemars(
        description = "lines beginning from - will be removed, lines beginning with + will be added, lines without any prefix will be unchanged"
    )]
    pub patch: String,
    #[schemars(description = "line number to start applying the patch")]
    pub start_line: usize,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct DiffFiles {
    pub file_0_path: String,
    pub file_1_path: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct DirLs {
    pub dir_path: String,
}

impl DirLs {
    pub fn signature(&self) -> String {
        format!("dir_ls: {}", self.dir_path)
    }
    pub fn exec(&self, project: Project) -> ActionResult {
        let project_dir = &project.dir;
        let file_path = project_dir.join(&project.cwd).join(&self.dir_path);

        if !project.path_is_allowed(&file_path) {
            return project.add_error(
                &self.signature(),
                "Access denied to dir -  outside root dir",
            );
        }
        let parent_dir = file_path.parent().unwrap();
        if !parent_dir.exists() {
            project.add_error(&self.signature(), "Error reading dir - dir not found");
        }

        match std::fs::read_dir(file_path) {
            Ok(entries) => {
                let mut result = String::new();
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let path = entry.path();
                            if path.is_dir() {
                                result.push_str(&format!("Directory: {}\n", path.display()));
                            } else {
                                result.push_str(&format!("File: {}\n", path.display()));
                            }
                        }
                        Err(_) => {
                            result.push_str("Error reading directory entry\n");
                        }
                    }
                }
                project.add_success(&self.signature(), &result)
            }
            Err(err) => project.add_error(
                &self.signature(),
                format!("Error reading dir - {:?}", err).as_str(),
            ),
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct CD {
    pub dir_path: String,
}

impl CD {
    pub fn signature(&self) -> String {
        format!("cd: {}", self.dir_path)
    }
    pub fn exec(&self, project: Project) -> ActionResult {
        let project_dir = &project.dir;
        let dir_path = project_dir.join(&self.dir_path);

        if !project.path_is_allowed(&dir_path) {
            return project.add_error(
                &self.signature(),
                "Access denied to dir -  outside root dir",
            );
        }

        if !dir_path.exists() {
            std::fs::create_dir_all(dir_path).unwrap();
            return project.add_success(&self.signature(), "Directory created successfully");
        }
        ActionResult {
            project,
            action: self.signature(),
            result: "Current Directory changed".to_string(),
            is_error: false,
            is_success: true,
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct Pwd {}
impl Pwd {
    pub fn signature(&self) -> String {
        "pwd".to_string()
    }
    pub fn exec(&self, project: Project) -> ActionResult {
        project.add_success(&self.signature(), &project.cwd)
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(tag = "action_type", rename_all = "snake_case")]
pub enum FsActions {
    #[schemars(description = "get content of a file")]
    ReadFile(ReadFile),
    #[schemars(description = "write content to a file, if file does not exist, create it")]
    WriteFile(WriteFile),
    #[schemars(description = "list content of a directory")]
    DirLs(DirLs),
    DiffFiles(DiffFiles),
    ApplyPatchToFile(ApplyPatchToFile),
    #[schemars(description = "change directory")]
    CD(CD),
    #[schemars(description = "get current directory")]
    Pwd(Pwd),
}
