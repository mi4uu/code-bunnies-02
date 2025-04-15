<<<<<<< SEARCH
=======
# Prompt Builder

A Rust library for building structured prompts for LLM agents that generate Rust programs.

## Features

- Builder pattern for creating prompts with various components
- Support for agents, projects, steps, and more
- Multiple output formats (Markdown, XML, Semi-XML)
- Custom formatting templates
- Ability to merge and reuse prompt components

## Usage Example

```rust
use prompt_builder::{Agent, Project, Prompt, Step, StepType};

fn main() {
    // Create an agent
    let agent = Agent::new()
        .with_name("Bob")
        .with_description("You are a Rust expert.")
        .with_agent_do("Write idiomatic rust code")
        .with_agent_dont("Create placeholders");

    // Create a project
    let project = Project::new()
        .with_name("calculator")
        .with_description("It makes numbers do things")
        .with_languages(vec![Project::WITH_RUST])
        .with_use("tokio");

    // Create a prompt
    let prompt_builder = Prompt::new()
        .with_agent(&agent)
        .with_project(&project)
        .with_steps(vec![
            Step::new(StepType::INSTRUCTION, "Create project"),
            Step::new(StepType::THINK, "Think about it for a moment"),
        ])
        .add_step(StepType::RUN, "ls -la")
        .add_step(StepType::ACT, "Implement program");

    // Build and output the prompt
    let prompt = prompt_builder.build();
    println!("{}", prompt);
}
