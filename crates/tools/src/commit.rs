mod prompt;
mod split_diff;
use spinoff::{spinners, Color, Spinner};
use std::process::Command;
use tracing::{info, warn};

// Maximum length for each diff part
const MAX_LEN_PER_DIFF: usize = 2000;

mod llm_response;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Get the staged git diff
    let git_diff = get_staged_diff()?;

    // Create a spinner for user feedback
    let mut sp = Spinner::new(spinners::Dots, "Generating commit message...", Color::Blue);

    // Split the diff into manageable parts
    let diff_parts: Vec<String> = split_diff::split_git_diff(&git_diff);
    
    if diff_parts.is_empty() {
        sp.stop();
        warn!("No changes detected in the staged files");
        return Err(anyhow::anyhow!("No changes to commit"));
    }

    info!("Generated {} diff parts", diff_parts.len());
    
    // Log part lengths for debugging
    let part_lengths: Vec<usize> = diff_parts.iter().map(|part| part.len()).collect();
    info!("Diff part lengths: {:?}", part_lengths);

    // Truncate large parts to the maximum allowed length
    let truncated_parts: Vec<String> = diff_parts
        .into_iter()
        .map(|part| {
            if part.len() > MAX_LEN_PER_DIFF {
                warn!("Truncating diff part from {} to {} characters", part.len(), MAX_LEN_PER_DIFF);
                part.chars().take(MAX_LEN_PER_DIFF).collect()
            } else {
                part
            }
        }).collect();
        
    // Generate commit message
    let commit_message = llm_response::get_commit_msg(truncated_parts)
        .await?;
    
    sp.stop();
    
    info!("Generated commit message: {}", &commit_message);
    create_commit(commit_message)
}

/// Creates a git commit with the provided message
fn create_commit(msg: String) -> anyhow::Result<()> {
    match Command::new("git").args(["commit", "-m", &msg]).status() {
        Ok(_) => Ok(()),
        Err(err) => Err(anyhow::anyhow!("Failed to create git commit: {}", err)),
    }
}

/// Gets the staged git diff
fn get_staged_diff() -> anyhow::Result<String> {
    // Stage all changes first
    Command::new("git")
        .args(["add", "."])
        .status()
        .map_err(|err| anyhow::anyhow!("Failed to stage changes: {}", err))?;
        
    // Get the diff of staged changes
    let diff_output = Command::new("git")
        .args(["diff", "--staged"])
        .output()
        .map_err(|err| anyhow::anyhow!("Failed to get diff: {}", err))?;
        
    String::from_utf8(diff_output.stdout)
        .map_err(|err| anyhow::anyhow!("Failed to parse diff output as UTF-8: {}", err))
}
