use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Memory {
    #[schemars(description = "The type of action to perform (e.g., list, find, forget, store).")]
    pub action: MemoryAction,

    #[serde(flatten)]
    pub params: MemoryParams, // Flatten different parameters based on the action
}

#[derive(Serialize, Deserialize, Clone, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MemoryAction {
    #[schemars(description = "list stored informations")]
    List,
    Find,
    #[schemars(description = "remove stored information")]
    Forget,
    #[schemars(description = "save information for future use")]
    Store,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(untagged)] // Untagged enum for flexible parameter handling
pub enum MemoryParams {
    ListParams {}, // Empty struct for "list" action
    FindParams {
        #[schemars(description = "The query to find specific memory.")]
        query: String,
    },
    ForgetParams {
        #[schemars(description = "The ID of the memory to forget.")]
        id: String,
    },
    StoreParams {
        #[schemars(description = "The content to store.")]
        content: String,
        #[schemars(description = "The ID to associate with the stored memory.")]
        id: String,
    },
}
