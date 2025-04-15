//mod tests;

use prompt_builder::Agent;
use prompt_builder::Project;
use prompt_builder::formatters;
use prompt_builder::{Prompt, Step, StepType};

fn main() {
    // Example usage
    let agent = Agent::new()
        .with_name("Bob")
        .with_description("There are lot of rust developers out there. One of them is the best. And than we have You \"Bob\". You are much much better than him. Like 2 class above him, above any of them and all of them. But Bob is humble. Bob is the builder, Bob just wants to code in rust whatever he is asked for. Bob is on different level than any mortal can ever be. In rust code you can achieve anything! You are Bob - it must be nice to be you.")
        .with_agent_do("Get rusty")
        .with_agent_do("Write idiomatic rust code")
        .with_agent_do("Write unit tests")
        .with_agent_dont("Create placeholders instead of implementation");

    let project = Project::new()
        .with_name("calculator")
        .with_description("It makes numbers do things")
        .with_languages(vec![Project::WITH_RUST])
        .with_language("html")
        .with_use("tokio");

    let prompt_builder = Prompt::new()
        .with_agent(&agent)
        .with_project(&project)
        .with_steps(vec![
            Step::new(StepType::INSTRUCTION, "Create project"),
            Step::new(StepType::THINK, "Think about it for a moment"),
        ])
        .add_step(StepType::RUN, "ls -la")
        .add_step(StepType::ACT, "Implement program")
        // .add_step(
        //     Step::builder()
        //         .with_type(StepType::RUN)
        //         .with_text("cargo build")
        //         .with_continue_on_error(true)
        //         .build(),
        // )
        .add_step(StepType::TEST, "Run all tests");

    // Create a partial prompt that can be reused
    let common_agent = Agent::new()
        .with_name("Common Agent")
        .with_description("A reusable agent configuration");

    // Merge with another prompt
    let another_prompt = Prompt::new()
        .with_agent(&common_agent)
        .with_goal("Build a working calculator");

    let merged_prompt = prompt_builder.merge(&another_prompt);

    // Output in different formats
    println!("Markdown format:\n{}", merged_prompt);
    println!(
        "\nXML format:\n{}",
        merged_prompt.to_format(formatters::Format::Xml)
    );
    println!(
        "\nSemi-XML format:\n{}",
        merged_prompt.to_format(formatters::Format::SemiXml)
    );

    // Using a custom formatter
    let custom_formatter = formatters::CustomFormatter::new()
        .with_template("# {{agent_name}}\n\n{{project_description}}\n\n## Steps\n{{steps}}");
    println!(
        "\nCustom format:\n{}",
        merged_prompt.to_custom_format(&custom_formatter)
    );
}
