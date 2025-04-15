use crate::{Agent, Project, Prompt, StepType};

fn main() {
    // Create an agent
    let agent = Agent::new()
        .with_name("Rust Expert")
        .with_description("You are a Rust programming expert who writes clean, idiomatic code.")
        .with_agent_do("Write well-documented code")
        .with_agent_do("Follow Rust best practices")
        .with_agent_dont("Use unsafe code unnecessarily");

    // Create a project
    let project = Project::new()
        .with_name("Web Server")
        .with_description("A simple HTTP server in Rust")
        .with_languages(vec![Project::WITH_RUST])
        .with_use("tokio")
        .with_use("hyper")
        .with_requirement("Must handle concurrent connections")
        .with_requirement("Should include error handling");

    // Create a prompt
    let prompt = Prompt::new()
        .with_agent(&agent)
        .with_project(&project)
        .with_goal("Create a working HTTP server that can serve static files")
        .add_step(StepType::INSTRUCTION, "Set up the project structure")
        .add_step(StepType::THINK, "Consider the architecture")
        .add_step(StepType::ACT, "Implement the server")
        // .add_step(
        //     Step::builder()
        //         .with_type(StepType::RUN)
        //         .with_text("cargo test")
        //         .with_continue_on_error(true)
        //         .build(),
        // )
        .with_rule("Code must be well-commented");

    // Print the prompt
    println!("{}", prompt);
}
