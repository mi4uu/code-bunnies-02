use regex::Regex;
use anyhow::Result;
use tracing::info;

// Constants used by the mistral implementation
const MODEL_ID: &str = "mradermacher/calme-3.3-llamaloi-3b-GGUF";
const MODEL_FILE: &str = "calme-3.3-llamaloi-3b.Q4_K_M.gguf";
const MODEL_TOKENIZER: &str = "MaziyarPanahi/calme-3.3-llamaloi-3b";

// Constants used by the OpenAI API implementation
#[cfg(not(feature = "mistral"))]
use std::env;

/// Default base URL for the OpenAI API
const DEFAULT_BASE_URL: &str = "https://api.openai.com/v1";

/// Default model to use for generating commit messages
const DEFAULT_MODEL: &str = "gpt-3.5-turbo";

/// Implementation of `get_commit_msg` using OpenAI API compatible services
/// 
/// This implementation is used when the `mistral` feature is disabled. It makes
/// HTTP requests to an OpenAI API compatible endpoint to generate commit messages
/// based on git diffs.
///
/// # Environment Variables
/// 
/// - `COMMIT_BASEURL`: Base URL for the API (defaults to "https://api.openai.com/v1")
/// - `COMMIT_MODEL`: Model to use for generating messages (defaults to "gpt-3.5-turbo")
/// - `COMMIT_API_KEY`: API key for the service (required)
///
/// # Parameters
/// 
/// - `diffs`: A vector of strings, each containing a git diff
///
/// # Returns
/// 
/// - `Result<String>`: A string containing the generated commit message(s), or an error
///
/// # Errors
/// 
/// - If `COMMIT_API_KEY` environment variable is not set
/// - If the API request fails
/// - If the API returns a non-success status code
/// - If the response cannot be parsed
#[cfg(not(feature = "mistral"))]
pub async fn get_commit_msg(diffs: Vec<String>) -> Result<String> {
    // Get configuration from environment variables with defaults
    let base_url = env::var("COMMIT_BASEURL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());
    let model = env::var("COMMIT_MODEL").unwrap_or_else(|_| DEFAULT_MODEL.to_string());
    let api_key = env::var("COMMIT_API_KEY").map_err(|_| {
        anyhow::anyhow!("COMMIT_API_KEY environment variable must be set for OpenAI API")
    })?;

    let mut responses: Vec<String> = Vec::new();

    for diff in diffs {
        let respclone = responses.clone();
        let allresp = respclone.join("\n");

        // Create message payload
        let messages = vec![
            serde_json::json!({
                "role": "system",
                "content": "You are an expert developer specializing in crafting clear, concise, and engaging commit messages based on `git diff` output"
            }),
            serde_json::json!({
                "role": "user",
                "content": get_prompt(diff, allresp)
            }),
        ];

        // Prepare request
        let client = reqwest::Client::new();
        let url = format!("{}/chat/completions", base_url);
        
        let request_body = serde_json::json!({
            "model": model,
            "messages": messages,
            "temperature": 0.7,
            "max_tokens": 1000
        });

        // Send request to OpenAI API
        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&request_body)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Request to API failed: {}", e))?;

        // Check for errors
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "<could not read error response>".to_string());
            return Err(anyhow::anyhow!(
                "API returned non-success status code {}: {}",
                status, error_text
            ));
        }

        // Parse the response
        let resp_json: serde_json::Value = response.json().await
            .map_err(|e| anyhow::anyhow!("Failed to parse API response: {}", e))?;

        // Extract the message content
        let content = resp_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid response format"))?
            .to_string();

        info!("AI response: {}", &content);
        
        // Extract the commit message from the content
        if let Some(commit_msg) = extract_commit_message(content) {
            info!("Extracted commit message: {}", &commit_msg);
            responses.push(commit_msg);
        }
    }
    
    // Join all commit messages with newlines and return
    Ok(responses.join("\n"))
}
#[cfg(feature = "mistral")]
pub async fn get_commit_msg(diffs: Vec<String>) -> Result<String> {
    use mistralrs::{GgufModelBuilder, TextMessageRole, TextMessages};
    
    // Initialize the model
    let model = GgufModelBuilder::new(MODEL_ID, vec![MODEL_FILE])
        .with_tok_model_id(MODEL_TOKENIZER)
        .with_logging()
        .with_logging()
        .with_max_num_seqs(100)
        .build()
        .await?;
    let mut responses: Vec<String> = Vec::new();

    for diff in diffs {
        let respclone = responses.clone();
        let allresp = respclone.join("\n");

        let messages = TextMessages::new()
        .add_message(
            TextMessageRole::System,
           "You are an expert developer specializing in crafting clear, concise, and engaging commit messages based on `git diff` output",
        ).add_message(
        TextMessageRole::User,
    get_prompt(diff, allresp)
    );
        let response = model.send_chat_request(messages.to_owned()).await?;
        let msg = response.choices[0].message.clone();
        let content = msg.content.unwrap();
        let r = content.clone();
        info!("AI response: {}", &r);
        if let Some(commit_msg) = extract_commit_message(content) {
            info!("Extracted commit message: {}", &commit_msg);
            responses.push(commit_msg);
        } else {
            info!(
                "No commit message found in the response, response was: {}",
                &r
            );
        }
    }
    let allresp = responses.join("\n");
    Ok(allresp)
}

/// Generates a prompt for the LLM to create a commit message from a git diff
/// 
/// This function takes a git diff and previous commit messages (if any) and 
/// creates a prompt that guides the LLM to generate a well-formatted commit message.
///
/// # Parameters
/// 
/// - `diff`: String containing the git diff
/// - `prev`: String containing previous commit messages (if any)
///
/// # Returns
fn get_prompt(diff: String, prev: String) -> String {
    let prompt = format!(
        r#"Here is the git diff result you need to analyze:

<git_diff>
{git_diff}
</git_diff>

use previous commit messages to help you understand the context of the changes:

<previous_commit_messages>
{previous_commit_messages}
</previous_commit_messages>

use the information above to not repeat same commit messages, and generate a commit message for the changes in the diff.

You are an experienced software developer tasked with generating informative commit messages based on git diff results. Your goal is to create clear, concise messages that accurately represent the changes made in the code.

Please follow these steps to generate an appropriate commit message:

1. Analyze the git diff carefully, paying attention to:
   - Files that were modified, added, or deleted
   - The nature of the changes (e.g., bug fixes, new features, refactoring)
   - The scope of the changes (e.g., affecting a single function, an entire class, or multiple files)

2. Based on your analysis, categorize the change. Common categories include:
   - feat: A new feature
   - fix: A bug fix
   - docs: Documentation changes
   - style: Code style changes (formatting, missing semi-colons, etc.)
   - ui: User interface changes
   - perf: Performance improvements
   - build: Changes to the build process or external dependencies
   - refactor: Code refactoring
   - test: Adding or modifying tests
   - chore: Updating build tasks, package manager configs, etc.

3. Select an appropriate emoji that represents the type of change. Some common examples:
   - ✨ (sparkles) for new features
   - 🐛 (bug) for bug fixes
   - 📚 (books) only for documentation changes
   - 🎨 (art) for style/formatting changes
   - ♻️ (recycle) for refactoring
   - ✅ (check mark) for adding tests
   - 🔧 (wrench) for configuration changes
   - 🚀 (rocket) for performance improvements
   - 🏗️ (building construction) for build system changes/ci/automation
   - ⏪️ (rewind) for reverting changes
   - ♻️ (recycle) for code refactoring
   - 🧪 (test probe) for tests changes 
   - 🔒 (lock) for security-related changes
   - 👁️ (eye) for ui changes


4. Formulate a commit message that summarizes the main change. The message should be clear and informative, and can span up to 3 lines if necessary. Include the following information:
   - The main purpose of the change
   - File names affected by the change (in bold)
   - Package, library, or crate information if applicable

Before providing your final commit message, wrap your analysis inside <diff_analysis> tags. In your analysis:
- List all modified, added, and deleted files.
- Consider multiple possible categories and emojis, listing pros and cons for each.
- Provide a brief summary of each significant change in the diff.
- Explain your final choice of category and emoji.

It's OK for this section to be quite long. Do not include any of this analysis in the final commit message.
Do not include any of this analysis in the response.

Finally, format your commit message as follows:

Output your result in the following format:
<commit_message>
[emoji] [category]: [concise description of the change]
        -  [additional details if necessary, up to 2 more lines as markdown list with 8 hard spaces indentation and proper emoji ]
</commit_message>

Example output structure:
<commit_message>
✨ feat: description of change **file_name_of_change** in **package_name**
         -  🔐 sumary of changes in more details
</commit_message>

Do not include example output in your response.
Respond only with commit message wrapped in <commit_message> tags. Do not include any other text or explanations.
Remember to keep the entire commit message as concise as possible while still conveying all necessary information.
"#,
        git_diff = diff,
        previous_commit_messages = prev
    );
    prompt
}

/// Extracts a commit message from the LLM response
/// 
/// This function uses regex to find content wrapped in `<commit_message>` tags
/// and extracts it as the commit message.
///
/// # Parameters
/// 
/// - `content`: String containing the LLM response
///
/// # Returns
/// 
/// - `Option<String>`: The extracted commit message or None if no commit message
///   was found in the content
pub fn extract_commit_message(content: String) -> Option<String> {
    // Use (?s) flag to make the dot match newlines as well
    let re = Regex::new(r"(?s)<commit_message>(.*?)</commit_message>").unwrap();

    // Check if there's a match for the <commit_message> tags
    if let Some(caps) = re.captures(content.as_str()) {
        // If there are captures, return the content within the <commit_message> tags
        if let Some(commit_msg) = caps.get(1) {
            // Trim whitespace from the extracted content
            return Some(commit_msg.as_str().trim().to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[cfg_attr(not(feature = "mistral"), ignore)]
    #[test]
    fn test_extract_commit_message() {
        let content = r#"
        <commit_message>
        ✨ feat: Added new feature in **file1.rs** in **package1**
                -  🔐 summary of changes in more details
        </commit_message>
        "#;

        let result = extract_commit_message(content.to_string());
        assert!(result.is_some());
        assert_eq!(
            result.unwrap(),
            "✨ feat: Added new feature in **file1.rs** in **package1**\n                -  🔐 summary of changes in more details"
        );
    }
    
    #[cfg_attr(not(feature = "mistral"), ignore)]
    #[tokio::test]
    async fn test_get_local_llm_response() {
        let example_diff = r#"diff --git a/src/main.rs b/src/main.rs
        index 1234567..89abcdef 100644
        --- a/src/main.rs
        +++ b/src/main.rs
        @@ -1,3 +1,4 @@
        +// Added a new feature
        fn main() {
        println!("Hello, world!"); }"#;
        let example2 = r#"diff --git a/crates/tools/src/commit.rs b/crates/tools/src/commit.rs
index 412c574..c678477 100644
--- a/crates/tools/src/commit.rs
+++ b/crates/tools/src/commit.rs
@@ -4,8 +4,7 @@ use core::panic;
 use std::{os::unix::fs::PermissionsExt, path::PathBuf};
 mod prompt;
 mod split_diff;
-use console::style;
-
+use spinoff::{spinners, Color, Spinner};
 use futures::future::join_all;
 // use minijinja::{Environment, context};
 use trauma::{
@@ -25,10 +24,11 @@ async fn main() -> anyhow::Result<()> {
     // tracing_subscriber::fmt::init();
     let diff = get_diff();
     
+    let mut sp = Spinner::new(spinners::Dots,"Generating commit message...", Color::Blue);
 
-    // let parts: Vec<String> = split_diff::split_git_diff(diff.as_str());
-    
-    let resp = llm_response::get_commit_msg(diff.as_str()).await.expect("generating commit summary failed");
+    let parts: Vec<String> = split_diff::split_git_diff(diff.as_str());
+    let resp = llm_response::get_commit_msg(parts).await.expect("generating commit summary failed");
+    sp.stop();
     //println!("Prompts: {:?}", prompts);
     // let final_response = join_all(
     //     parts"#;

        let start = std::time::Instant::now();
        let response = get_commit_msg(vec![example_diff.to_string(), example2.to_string()]).await;

        assert!(&response.is_ok(), "The response should be Ok");
        let response_text = response.unwrap();
        println!("--------------------------------");
        println!("Response: {}", &response_text);
        println!("--------------------------------");
        assert!(
            !response_text.is_empty(),
            "The response text should not be empty"
        );
        println!("{:?}ms", start.elapsed().as_millis());
    }
}

#[cfg(test)]
#[cfg(not(feature = "mistral"))]
mod openai_tests {
    use super::{extract_commit_message, get_prompt};

    #[test]
    fn test_extract_commit_message() {
        let content = r#"
<commit_message>
✨ feat: Added new feature in **file1.rs** in **package1**
        -  🔐 summary of changes in more details
</commit_message>
"#;

        let result = extract_commit_message(content.to_string());
        assert!(result.is_some());
        assert_eq!(
            result.unwrap(),
            "✨ feat: Added new feature in **file1.rs** in **package1**\n        -  🔐 summary of changes in more details"
        );
    }

    #[test]
    fn test_get_prompt_format() {
        let diff = "diff --git a/src/test.rs b/src/test.rs\nnew file mode 100644";
        let prev = "Previous commit message";

        let prompt = get_prompt(diff.to_string(), prev.to_string());

        // Check that the prompt contains the expected sections
        assert!(prompt.contains("<git_diff>"));
        assert!(prompt.contains(diff));
        assert!(prompt.contains("<previous_commit_messages>"));
        assert!(prompt.contains(prev));

        // Check that it includes the instructions
        assert!(prompt.contains("generate a commit message"));
        assert!(prompt.contains("<commit_message>"));
    }

    // Tests for edge cases

    #[test]
    fn test_empty_diff() {
        let empty_diff = "";
        let prompt = get_prompt(empty_diff.to_string(), "".to_string());

        // Ensure the prompt handles empty diffs properly
        assert!(prompt.contains("<git_diff>\n\n</git_diff>"));
        assert!(prompt.contains("generate a commit message"));
    }

    #[test]
    fn test_large_diff() {
        // Create a diff that's larger than MAX_LEN_PER_DIFF
        let large_line = "+".to_owned() + &"a".repeat(1000);
        let large_diff = format!(
            "diff --git a/large.txt b/large.txt\n--- a/large.txt\n+++ b/large.txt\n@@ -1,0 +1,3 @@\n{}\n{}\n{}\n",
            large_line, large_line, large_line
        );

        // The large diff should be over 3000 characters
        assert!(large_diff.len() > 3000);

        let prompt = get_prompt(large_diff.clone(), "".to_string());

        // Ensure the prompt contains the entire large diff
        assert!(prompt.contains(&large_diff));
    }

    #[test]
    fn test_multiple_diffs() {
        let diff1 = "diff --git a/file1.txt b/file1.txt\n--- a/file1.txt\n+++ b/file1.txt\n@@ -1 +1 @@\n-old\n+new";
        let diff2 = "diff --git a/file2.txt b/file2.txt\n--- a/file2.txt\n+++ b/file2.txt\n@@ -1 +1 @@\n-foo\n+bar";

        // We're testing the prompt functionality here, which should handle a single diff
        // The commit.rs handles splitting and combining diffs
        let prompt = get_prompt(format!("{}\n{}", diff1, diff2), "".to_string());

        // Ensure the prompt contains both diffs
        assert!(prompt.contains(diff1));
        assert!(prompt.contains(diff2));
    }
}
