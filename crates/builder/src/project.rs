use std::path::PathBuf;

use names::Generator;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct Project {
    pub name: String,
    pub dir: PathBuf,
    pub cwd: String,
}
impl Default for Project {
    fn default() -> Self {
        Self::new()
    }
}

impl Project {
    pub fn new() -> Self {
        let mut generator = Generator::default();

        let output_dir = env!("PLAYGROUND_DIR");
        let project_name = generator.next().unwrap();
        let project_path = PathBuf::from(output_dir).join(&project_name);
        std::fs::create_dir_all(&project_path).unwrap();
        Self {
            name: project_name,
            dir: project_path,
            cwd: "/".to_string(),
        }
    }
    pub fn path_is_allowed(&self, path: &PathBuf) -> bool {
        let project_dir = &self.dir;
        let file_path = project_dir.join(path);
        let abs_file_path = file_path.canonicalize().unwrap_or_else(|_| PathBuf::new());
        abs_file_path.starts_with(project_dir)
    }
    pub fn add_success(&self, action: &str, result: &str) -> ActionResult {
        ActionResult {
            project: self.clone(),
            action: action.to_string(),
            result: result.to_string(),
            is_error: false,
            is_success: true,
        }
    }
    pub fn add_error(&self, action: &str, result: &str) -> ActionResult {
        ActionResult {
            project: self.clone(),
            action: action.to_string(),
            result: result.to_string(),
            is_error: true,
            is_success: false,
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct ActionResult {
    pub project: Project,
    pub action: String,
    pub result: String,
    pub is_error: bool,
    pub is_success: bool,
}
