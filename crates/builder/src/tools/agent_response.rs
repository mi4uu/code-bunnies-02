use super::fs::FsActions;
use super::memory::MemoryAction;
use schemars::JsonSchema;
use schemars::schema_for;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, JsonSchema, Debug)]
pub struct SearchWeb {
    pub query: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct UserAssistanceNeeded {
    pub message: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, strum_macros::EnumIs)]
#[serde(tag = "action_type", rename_all = "snake_case")]
pub enum AgentActions {
    #[schemars(description = "filesystem actions")]
    Fs(FsActions),
    #[schemars(description = "serch the web for information")]
    SearchWeb(SearchWeb),
    #[schemars(description = "user input is required")]
    UserAssistanceNeeded(UserAssistanceNeeded),
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, strum_macros::EnumIs)]
#[serde(tag = "action_type", rename_all = "snake_case")]
pub enum AgentTools {
    #[schemars(description = "agent memory")]
    Memory(MemoryAction),
}


#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct AgentResponse {
    #[schemars(
        description = "The reasoning or explanation behind the chosen tasks. use step by step thinking"
    )]
    pub reasoning: Vec<String>,
    #[schemars(description = "The actions that the agent will perform.")]
    pub actions: AgentActions,
    #[schemars(description = "The tools help the agent manage the context and memory")]
    pub tools: Vec<AgentTools>,
    #[schemars(description = "actions summary")]
    pub summary: Option<String>,
    #[schemars(description = "actions expected results")]
    pub expected: Option<String>,
    #[schemars(
        description = "plan: what the agent will do next"
    )]
    pub next: String,
}
impl AgentResponse {
    pub fn schema() -> serde_json::Value {
        let schema = schema_for!(Self);
        serde_json::json!(schema)
    }
    pub fn format() -> String {
        let schema = schema_for!(Self);
        // serde_json::json!(schema).as_object().unwrap();
        serde_json::to_string_pretty(&schema).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::providers::openai::OpenAiProvider;

    use super::*;
    #[test]
    fn test_show_all() {
        let schema = AgentResponse::format();
        println!("{}", schema);
    }

    #[tokio::test]
    async fn test_memory() {
        use crate::providers::openai::ChatMessage;
        use crate::providers::providers::Providers;
        use crate::tools;
        use anyhow;
        use tempdir::TempDir;
        let tmp_dir = TempDir::new("memory-test").unwrap();
        let providers = Providers::load();
        let lm_studio = providers
            .get_by_name("LmStudio")
            .expect("LmStudio provider not found");
        let systemprompt = r#"You are a helpful assistant. 
        You can perform various tasks, including file operations, memory management, and web searches. 
        Please provide clear instructions for the tasks you want to perform.
        remember that results of those tasks will be avilable after you respond and not before. 
        You can also ask the user for input if needed, but only once and as an last action. plan your tasks step by step. 
        if you need to ask the user for input, do it at the end of your response. multiple user assistance needed actions are not allowed and will be ignored.
        if you need to perform steps that depends on other steps or informations you can use the memory actions to store and retrieve information including steps, progress, and future actions.
        
        "#;
        let prompt = r#"
        
        You are a helpful assistant. 
        You can perform various tasks, including file operations, memory management, and web searches. 
        Please provide clear instructions for the tasks you want to perform.
        remember that results of those tasks will be avilable after you respond and not before. 
        You can also ask the user for input if needed, but only once and as an last action. plan your tasks step by step. 
        if you need to ask the user for input, do it at the end of your response. multiple user assistance needed actions are not allowed and will be ignored.
        if you need to perform steps that depends on other steps or informations you can use the memory actions to store and retrieve information including steps, progress, and future actions.
        
        steps:
        - ask user for 1 number input
        - remember the number
        - ask user for second number input
        - get save second number
        - ask user for operation
        - remember the operation
        - return the result of the operation
        "#;
        let provider = lm_studio
            .with_model("bartowski/Tesslate_Gradience-T1-3B-preview-GGUF")
            .unwrap()
            .with_temperature(0.1) // Lower temperature for more deterministic answers
            .with_max_tokens(1000) // Limit response length
            .with_top_p(0.95)
            .with_ctx(9000) // Slightly adjust nucleus sampling
            .build()
            .unwrap();

        // Create messages for the chat completion
        let messages = vec![ChatMessage::system(systemprompt), ChatMessage::user(prompt)];
        let r = ask(provider.clone(), messages).await.unwrap();
        for a in r.clone().actions.into_iter() {
            println!("====================");
            match a {
                AgentActions::Memory(m) => {
                    println!("Memory action: {:?}", m);
                }
                AgentActions::Fs(f) => {
                    println!("File system action: {:?}", f);
                }
                AgentActions::SearchWeb(s) => {
                    println!("Search web action: {:?}", s);
                }
                AgentActions::UserAssistanceNeeded(u) => {
                    println!("User assistance needed: {:?}", u);
                }
                _ => {
                    println!("Unknown action: {:?}", a);
                }
            }
        }
        let messages = vec![
            ChatMessage::system(systemprompt),
            ChatMessage::user(prompt),
            ChatMessage::assistant(serde_json::json!(r).to_string()),
            ChatMessage::user("first number: 1"),
        ];
        let r = ask(provider, messages).await.unwrap();
        for a in r.clone().actions.into_iter() {
            println!("====================");
            match a {
                AgentActions::Memory(m) => {
                    println!("Memory action: {:?}", m);
                }
                AgentActions::Fs(f) => {
                    println!("File system action: {:?}", f);
                }
                AgentActions::SearchWeb(s) => {
                    println!("Search web action: {:?}", s);
                }
                AgentActions::UserAssistanceNeeded(u) => {
                    println!("User assistance needed: {:?}", u);
                }
                _ => {
                    println!("Unknown action: {:?}", a);
                }
            }
        }
        // Query the model
        async fn ask(
            provider: OpenAiProvider,
            messages: Vec<ChatMessage>,
        ) -> anyhow::Result<AgentResponse> {
            let askr = match provider
                .chat_completion(
                    messages,
                    None,
                    None,
                    None,
                    Some(tools::agent_response::AgentResponse::schema()),
                )
                .await
            {
                Ok(response) => {
                    println!("Response from: {}", response);
                    let r: AgentResponse = match serde_json::from_str(&response) {
                        Ok(r) => {
                            println!("Parsed response: {:?}", r);
                            r
                        }
                        Err(e) => {
                            println!("Error parsing response: {}", e);
                            return Err(anyhow::anyhow!("Error parsing response"));
                        }
                    };
                    Ok(r)
                }
                Err(e) => {
                    println!("Error {}", e);
                    Err(e)
                }
            };
            askr.map_err(|e| anyhow::anyhow!(e))
        }
    }
}
// fn main() {
//     // Generate schemas for all structs
//     let read_file_schema = schema_for!(ReadFile);
//     let write_file_schema = schema_for!(WriteFile);
//     let dir_ls_schema = schema_for!(DirLs);
//     let search_web_schema = schema_for!(SearchWeb);
//     let agent_response_schema = schema_for!(AgentResponse);

//     // Print the schemas
//     println!("ReadFile Schema:\n{}", serde_json::to_string_pretty(&read_file_schema).unwrap());
//     println!("WriteFile Schema:\n{}", serde_json::to_string_pretty(&write_file_schema).unwrap());
//     println!("DirLs Schema:\n{}", serde_json::to_string_pretty(&dir_ls_schema).unwrap());
//     println!("SearchWeb Schema:\n{}", serde_json::to_string_pretty(&search_web_schema).unwrap());
//     println!(
//         "AgentResponse Schema:\n{}",
//         serde_json::to_string_pretty(&agent_response_schema).unwrap()
//     );
// }
