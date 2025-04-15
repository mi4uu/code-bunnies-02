//! Module for splitting Git diff messages.

/// Splits a Git diff message string into a vector of strings, where each element
/// represents the diff for a single file.
///
/// It identifies file boundaries by looking for lines starting with "diff --git ".
/// Each element in the output vector will contain the complete diff section for one file,
/// including the initial "diff --git " line.
///
/// Any content in the input string *before* the first "diff --git " line is ignored,
/// as it's typically not part of a specific file's diff hunk.
///
/// # Arguments
///
/// * `msg`: The input string containing the Git diff output.
///
/// # Returns
///
/// A `Vec<String>` where each string is a chunk corresponding to a single file's diff.
/// - Returns an empty vector if the input string is empty or contains no "diff --git " lines.
///
/// # Example
///
/// ```rust
/// let diff_output = r#"diff --git a/file1.txt b/file1.txt
/// index 0000000..1111111 100644
/// --- a/file1.txt
/// +++ b/file1.txt
/// @@ -1,1 +1,1 @@
/// -Hello
/// +Hello World
/// diff --git a/src/main.rs b/src/main.rs
/// index 2222222..3333333 100644
/// --- a/src/main.rs
/// +++ b/src/main.rs
/// @@ -5,3 +5,4 @@
///  fn main() {
///      println!("Example");
///  }
/// +// New comment
/// "#;
/// let file_diffs = split_git_diff(&diff_output);
/// assert_eq!(file_diffs.len(), 2);
/// assert!(file_diffs[0].starts_with("diff --git a/file1.txt"));
/// assert!(file_diffs[0].contains("+Hello World"));
/// assert!(file_diffs[1].starts_with("diff --git a/src/main.rs"));
/// assert!(file_diffs[1].contains("+// New comment"));
/// ```
pub fn split_git_diff(msg: &str) -> Vec<String> {
    if msg.is_empty() {
        return Vec::new();
    }

    let mut result: Vec<String> = Vec::new();
    // Use Vec<&str> for efficiency, join lines only when a chunk is complete.
    let mut current_chunk_lines: Vec<&str> = Vec::new();
    const DELIMITER: &str = "diff --git ";

    for line in msg.lines() {
        if line.starts_with(DELIMITER) {
            // If we encounter a delimiter and we have lines accumulated for the *previous* file...
            if !current_chunk_lines.is_empty() {
                // Join the lines of the previous chunk and push it to results.
                result.push(current_chunk_lines.join("\n"));
            }
            // Start the new chunk *with* the delimiter line.
            current_chunk_lines = vec![line];
        } else if !current_chunk_lines.is_empty() {
            // Only add lines if we are *already inside* a chunk (i.e., after the first delimiter).
            // This implicitly ignores any text before the first "diff --git " line.
            current_chunk_lines.push(line);
        }
        // If current_chunk_lines is empty and the line doesn't start with the delimiter,
        // it means we are before the first actual diff chunk, so we ignore the line.
    }

    // After the loop, if there's any remaining content in the last chunk, push it.
    if !current_chunk_lines.is_empty() {
        result.push(current_chunk_lines.join("\n"));
    }

    result
}

// --- Unit Tests ---
#[cfg(test)]
mod tests {
    use super::*; // Import items from parent module

    // Helper function to create Vec<String> easily for comparison
    fn vec_s(strs: &[&str]) -> Vec<String> {
        strs.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(split_git_diff(""), Vec::<String>::new());
    }

    #[test]
    fn test_no_diff_lines() {
        let input = "Just some regular text.\nNo diff markers here.";
        assert_eq!(split_git_diff(input), Vec::<String>::new());
    }

    #[test]
    fn test_single_file_diff() {
        let input = r#"diff --git a/my_script.py b/my_script.py
index abcdef0..1234567 100755
--- a/my_script.py
+++ b/my_script.py
@@ -1,3 +1,4 @@
 import sys
 print("Hello")
 # Old comment
+# New comment"#;
        let expected = vec_s(&[input]);
        assert_eq!(split_git_diff(input), expected);
    }

    #[test]
    fn test_multiple_file_diffs() {
        let input = r#"diff --git a/file1.txt b/file1.txt
index 000..111 100644
--- a/file1.txt
+++ b/file1.txt
@@ -1 +1 @@
-a
+b
diff --git a/file2.rs b/file2.rs
new file mode 100644
index 000..222
--- /dev/null
+++ b/file2.rs
@@ -0,0 +1 @@
+fn test() {}"#;
        let expected = vec_s(&[
            r#"diff --git a/file1.txt b/file1.txt
index 000..111 100644
--- a/file1.txt
+++ b/file1.txt
@@ -1 +1 @@
-a
+b"#,
            r#"diff --git a/file2.rs b/file2.rs
new file mode 100644
index 000..222
--- /dev/null
+++ b/file2.rs
@@ -0,0 +1 @@
+fn test() {}"#,
        ]);
        assert_eq!(split_git_diff(input), expected);
    }

    #[test]
    fn test_preamble_text_is_ignored() {
        let input = r#"This is some text before the diff.
It should be ignored.

diff --git a/README.md b/README.md
--- a/README.md
+++ b/README.md
@@ -1 +1 @@
-Old Readme
+New Readme
diff --git a/main.go b/main.go
--- a/main.go
+++ b/main.go
@@ -1 +1,2 @@
 package main
+import "fmt""#;
        let expected = vec_s(&[
            r#"diff --git a/README.md b/README.md
--- a/README.md
+++ b/README.md
@@ -1 +1 @@
-Old Readme
+New Readme"#,
            r#"diff --git a/main.go b/main.go
--- a/main.go
+++ b/main.go
@@ -1 +1,2 @@
 package main
+import "fmt""#,
        ]);
        assert_eq!(split_git_diff(input), expected);
    }

    #[test]
    fn test_trailing_newline_input() {
        let input = r#"diff --git a/file1.txt b/file1.txt
--- a/file1.txt
+++ b/file1.txt
@@ -1 +1 @@
-line
+line fixed
diff --git a/file2.txt b/file2.txt
--- a/file2.txt
+++ b/file2.txt
@@ -1 +1 @@
-another
+another fixed

"#; // Note the trailing newline
        let expected = vec_s(&[
            r#"diff --git a/file1.txt b/file1.txt
--- a/file1.txt
+++ b/file1.txt
@@ -1 +1 @@
-line
+line fixed"#,
            r#"diff --git a/file2.txt b/file2.txt
--- a/file2.txt
+++ b/file2.txt
@@ -1 +1 @@
-another
+another fixed"#,
        ]);
        assert_eq!(split_git_diff(input), expected);
    }

    #[test]
    fn test_example_from_prompt() {
        let input = r#"diff --git a/crates/tools/src/commit.rs b/crates/tools/src/commit.rs
index 7246843..2f7005c 100644
--- a/crates/tools/src/commit.rs
+++ b/crates/tools/src/commit.rs
@@ -76,50 +76,7 @@ async fn main() -> anyhow::Result<()> {

 async fn get_ai_response(msg: String) -> String {
     
-
-
-    info!("AI prompt: {}", msg);
-    let model_dir = env!("OUT_DIR");
-   
-
-    // let model_name = "granite-3.2-8b-instruct";
-    // let quant_name = "-Q4_K_M";
-    let model_path = PathBuf::from(model_dir)
-        .join("models")
-        .join(format!("{MODEL_NAME}.llamafile"));
-    if !model_path.exists() {
-        donwload_model(model_path.clone()).await.unwrap();
-    }
-    let m=shlex::try_quote(&msg).unwrap().to_string();
-    info!("Quoted AI prompt: {}", &m);
-    let prompt_file= PathBuf::from(model_dir)
-    .join("models")
-    .join(format!("prompt.txt"));
-    fs::write(&prompt_file, m).expect("Failed to write prompt file");
-   // let escaped_msg: String =  shlex::split(msg.as_str()).unwrap().join(" ");
-    let resp = Command::new(model_path)
-        .arg("-f")
-        .arg(fs::canonicalize(prompt_file.as_os_str()).unwrap())
-        .arg("--no-display-prompt")
-        .arg("--repeat-penalty").arg("1.3").arg("-c").arg("8000")
-        .output();
-    match resp {
-        Ok(resp) => {
-            if resp.status.success() {
-                let r =  String::from_utf8_lossy(&resp.stdout);
-
-        //        println!("AI response: {:#?}",&r);
-                return String::from(r);
-            } else {
-            //    println!("AI error: {:#?}", String::from_utf8_lossy(&resp.stderr));
-            
-                panic!("AI error: {:#?}", String::from_utf8_lossy(&resp.stderr));
-            }
-        },
-        Err(err) => {
-            info!("AI error: {}", &err);
-            panic!("AI error: {:#?}", err);
-
-        }
+    String::new()
     }
 
     //     let response_text = String::from_utf8_lossy(&resp.unwrap().stdout);
@@ -149,88 +106,7 @@ fn get_diff() -> String {
     String::from_utf8(diff_buf).unwrap()
 }
 
-fn parse_stream_response(response_text: &str) -> String {
-    response_text.split('\n').filter_map(parse_line).collect()
-}
-fn parse_line(line: &str) -> Option<String> {
-    let data = line.strip_prefix("data: ")?; // Extract data after "data: " prefix
-    let json_value: Value = serde_json::from_str(data).ok()?;
-"#;
        let result = split_git_diff(input);
        // In this specific example, there's only one file diff
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], input); // The whole input is the single diff chunk
    }
}
