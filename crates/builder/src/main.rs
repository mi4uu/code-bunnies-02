use std::time::Duration;

use cb_builder::providers::openai::ChatMessage;
use cb_builder::providers::providers::Providers;
use cb_builder::tools;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load providers configuration
    let providers = Providers::load();
    println!("Loaded {} providers", providers.providers.len());

    // Get the LmStudio provider
    let lm_studio = providers
        .get_by_name("LmStudio")
        .expect("LmStudio provider not found");
    println!("Using provider: {}", lm_studio.name);
    println!("Available models: {:?}\n", lm_studio.models);

    // Define our simple prompt
    let prompt = "write program in rust that sums 2 numbers and print the result";
    println!("Prompt: {}", prompt);
    println!("-----------------------------");
    println!("schema: {}", tools::agent_response::AgentResponse::schema());
    // Example with a single model and customized parameters
    if let Some(model_builder) =
        lm_studio.with_model("jbaron34_-_qwen2.5-0.5b-bebop-reranker-newer-small")
    {
        println!("\nQuerying model: granite3.2 with custom parameters");

        // Use the builder pattern to customize parameters
        let provider = model_builder
            .with_temperature(0.3) // Lower temperature for more deterministic answers
            .with_max_tokens(100) // Limit response length
            .with_top_p(0.95)
            .with_ctx(9000) // Slightly adjust nucleus sampling
            .build()?;

        // Create messages for the chat completion
        let messages = vec![ChatMessage::user(prompt)];

        // Query the model
        match provider
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
                println!("Response from granite3.2 (custom params): {}", response);
            }
            Err(e) => {
                println!("Error from granite3.2: {}", e);
            }
        }
    } else {
        println!("Model 'granite3.2' not found");
    }

    // For each model in the LmStudio provider, create a request and get a response
    for model_entry in &lm_studio.models {
        println!("\nQuerying model: {}", model_entry.name);

        // Get a model builder and build the provider with default settings
        let provider = match lm_studio.with_model(&model_entry.name) {
            Some(builder) => match builder.build() {
                Ok(provider) => provider,
                Err(e) => {
                    println!("Error building provider for {}: {}", model_entry.name, e);
                    continue;
                }
            },
            None => {
                println!("Error: Model {} not found", model_entry.name);
                continue;
            }
        };

        // Create messages for the chat completion
        let messages = vec![ChatMessage::user(prompt)];

        // Query the model with a timeout
        let result = tokio::time::timeout(
            Duration::from_secs(30), // 30 second timeout
            provider.chat_completion(
                messages,
                None,
                None,
                None,
                Some(tools::agent_response::AgentResponse::schema()),
            ),
        )
        .await;

        // Handle the result
        match result {
            Ok(Ok(response)) => {
                println!("Response from {}: {}", model_entry.name, response);
            }
            Ok(Err(e)) => {
                println!("Error from {}: {}", model_entry.name, e);
            }
            Err(_) => {
                println!("Timeout waiting for response from {}", model_entry.name);
            }
        }
    }

    Ok(())
}
