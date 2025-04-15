# instruction

create a rust program that will help me build perfect prompts for llm agents
that generates other rust programs.

# requirements

- should be using builder pattern
- should have predefined parts like:
    - agent
        - name
        - description
        - agent_do
        - agent_dont
    - project
        - project_name
        - description
        - requirements
        - rules
        - languages
        - use
        - dont_use
        - good_example
        - bad_example
        - goal
    - steps
        - type (instruction, run, ask, think, ...)
        - text
        - action_on_error
        - action_on_success


## fields explanation:

- goal
    what result in the end will classify results as success
- steps
    ordered list of steps that we want for agent to do in that
- use
    list of library / packages that we want to be used in project
- languages
    programming language, might be a list of allowed ones like golang, rust, javascript, typescript, css

# results

results should give us some instance of struct that we can export to predefined formats.
we should be able to define multiple templates for those formats. if none is speified
we should default markdown template to use .

from result we can :
    - get prompt in any of prededefined formats
        -   default one should be just markdown with lists and headers
        -   other options should be easy to add
        - buildin option will be simple markdown (default)
        - xml  (without xml header but all properties should be in xml tags that will be named in a way that llm agent will understand purpose of that section)
        - semi-xml ( only some sections like directives <bad_example></bad_example> etc. most of prompt will be a markdown)
    - get system message for agent as markdown

- every element in that builder shoud be optional and skipped if not specified
- all elements (where it make sense, not like agent name ) should have some predefined options with optionall variables to set, program should be aware of those options and be able to use intellisens to discover those Options
- all elements with options to choose should also allow to add custom one.

# example of usage

```rust

let agent=Agent::new().with_name("Bob").with_description("there are lot of rust developers out there. one of them is the best. and than we have You \"Bob\". You are much much better than him. like 2 class above him, above any of them and all of them. But Bob is humble. Bob is the builder, Bob just wany to code in rust whatever he is asked for. Bob is on differen level than any mortal can ever be. in rust code you can acheive anything! You are Bob - it must be nice to be you.").with_agent_do("get rusty").with_agent_do("write idiomatic rust code").with_agent_do("write unit tests").with_agent_dont("create placeholders instead of implementation");

let project = Project::new().with_name("calculator").with_description("it make numbers do things").with_languages(Project::WITH_RUST).with_languages("html").with_use("tokio");


let prompt_builder=Prompt::new().with_agent(&agent).with_project(&project).with_steps(vec!["create project","think about it for a moment"]).add_step("run","ls -la").add_step("act","implement program").add_step(..Prompt::Step::builder().run("cargo build").with_continue_on_error()).add_step(Prompt::Step::TEST, "run all tests");

let prompt=prompt_builder.build();
let prompt_md=prompt.to_string();
let prompt_latex=prompt.to_custom_format(Prompt::FormatAsLatex);

```

## sugested libs usage

- bon

## plan

think how to make it more ergonomic and improve developer experience.

# expected result

- similar to example of usage, does not have to be 100% accurate with provided example.
- try to think how to make it even more ergonomic than what is showed on example.
- should allow user to define partial prompt instance and then merge them.
- should allow to easy define helpers to not repeat commonly used parts and provide generic partials.
- allow to create custom output formatters/templates.

# important
implement whole program, not high level design or something not working.
use functions to implement program