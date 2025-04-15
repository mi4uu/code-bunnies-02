use crate::{
    Agent, Project, Prompt,
    formatters::{CustomFormatter, Format},
};

fn main() {
    // Create a reusable agent template
    let rust_expert = Agent::new()
        .with_name("Rust Expert")
        .with_description("You are a Rust programming expert who writes clean, idiomatic code.")
        .with_agent_dos(vec![
            "Write well-documented code",
            "Follow Rust best practices",
            "Include unit tests",
        ])
        .with_agent_donts(vec![
            "Use unsafe code unnecessarily",
            "Ignore error handling",
        ]);

    // Create a reusable project template
    let web_project = Project::new()
        .with_languages(vec![Project::WITH_RUST])
        .with_uses(vec!["tokio", "hyper", "serde"])
        .with_requirement("Must handle concurrent connections")
        .with_requirement("Should include error handling");

    // Create a specific prompt using templates
    let server_project = Project::new()
        .with_name("Web Server")
        .with_description("A simple HTTP server in Rust")
        .merge(&web_project);

    let prompt = Prompt::new()
        .with_agent(&rust_expert)
        .with_project(&server_project)
        .with_goal("Create a working HTTP server that can serve static files")
        .add_step(Prompt::INSTRUCTION, "Set up the project structure")
        .add_step(Prompt::THINK, "Consider the architecture")
        .add_step(Prompt::ACT, "Implement the server")
        .add_step(Prompt::RUN, "cargo test");

    // Print in different formats
    println!("Markdown format:\n{}", prompt);
    println!("\nXML format:\n{}", prompt.to_format(Format::Xml));

    // Use a custom formatter
    let custom_formatter = CustomFormatter::new()
        .with_template("# {{project_name}}\n\n{{project_description}}\n\nYour task is to {{goal}}.\n\n## Steps\n{{steps}}")
        .with_variable("extra_note", "Take your time and be thorough.");

    println!(
        "\nCustom format:\n{}",
        prompt.to_custom_format(&custom_formatter)
    );
}
