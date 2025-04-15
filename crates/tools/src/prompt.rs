use minijinja::Environment;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct PromptParams {
    system_message: String,
    instruction: String,
    role: String,
    example_input: Option<String>,
    bad_example_output: Option<String>,
    example_output: Option<String>,
    input: String,
}

fn create_prompt(params: &PromptParams, template_name: &str) -> Result<String, minijinja::Error> {
    let mut env = Environment::new();
    env.add_template("tiny", r#"<|im_start|>system
{{system_message}}<|im_end|>
<|im_start|>user
## Role
{{ role }}

## Instruction
{{ instruction }}

## Input
```
{{ input }}
```


## Instruction
write commit message for the provided diff chunk
## OUTPUT:
Generate *only* the commit message based on the provided diff chunk and the requirements above<|im_end|>
<|im_start|>assistant
"#);
    // Define templates
    env.add_template(
        "inst",
        r#"
[INST]
## System Message
<system>{{ system_message }}</system>
## Role
{{ role }}

## Instruction
{{ instruction }}

## Input
```
{{ input }}
```


## Instruction
write commit message for the provided diff chunk

## OUTPUT:
Generate *only* the commit message based on the provided diff chunk and the requirements above
[/INST]"#,
    )?;

    env.add_template(
        "markdown",
        r#"
## System Message
{{ system_message }}

## Role
{{ role }}

## Instruction
{{ instruction }}

## Input
```
{{ input }}
```

{% if example_input is defined %}
## Example Input
```
{{ example_input }}
```

## Example Good Output
```
{{ example_output }}
```


## Example of BAD Output
```
{{ bad_example_output }}
```


{% endif %}
"#,
    )?;

    env.add_template(
        "tags",
        r#"<system>{{ system_message }}</system>
<role>{{ role }}</role>
<instruction>{{ instruction }}</instruction>

<input>
{{ input }}
</input>

{% if example_input is defined %}
<example-input>
{{ example_input }}
</example-input>
{% endif %}


{% if example_output is defined %}
<example-output>
{{ example_output }}
</example-output>
{% endif %}

{% if bad_example_output is defined %}
<bad-example-output>
{{ bad_example_output }}
</bad-example-output>
{% endif %}

"#,
    )?;

    let ctx = minijinja::context! {
        system_message => params.system_message,
        role => params.role,
        instruction => params.instruction,
        input => params.input,
        example_input => params.example_input,
        example_output => params.example_output,
        bad_example_output => params.bad_example_output,
    };
    Ok(env
        .get_template(template_name)
        .unwrap()
        .render(ctx)
        .unwrap())
    // env.render_str(template_name, ctx)
    // env.render_template(template_name, params)
}

pub enum Format {
    Inst,
    Markdown,
    Tags,
    Tiny,
}
impl Format {
    pub fn to_string(&self) -> String {
        match self {
            Format::Tiny => "tiny".to_string(),
            Format::Inst => "inst".to_string(),
            Format::Markdown => "markdown".to_string(),
            Format::Tags => "tags".to_string(),
        }
    }
}
pub fn get_prompt(msg: String, format: Format) -> Result<String, minijinja::Error> {
    let params = PromptParams {
        system_message: String::from(
            "You are an expert developer specializing in crafting clear, concise, and engaging commit messages based on `git diff` output",
        ),
        instruction: String::from(
            r#"
1.  **Emoji Prefix:** Start the subject line with an appropriate emoji to signify the primary nature of the change in *this chunk*:
    *   ✨ `feat:` (New Feature)
    *   🐛 `fix:` (Bug Fix)
    *   ♻️ `refactor:` (Code Refactoring without behavior change)
    *   ⚡️ `perf:` (Performance Improvement)
    *   📝 `docs:` (Documentation updates)
    *   🎨 `style:` (Code Style/Formatting changes)
    *   🧪 `test:` (Adding or updating tests)
    *   🏗️ `build:` (Build system, CI, or dependency changes)
    *   🚀 `chore:` (Routine maintenance, tooling, or other non-code changes)
    *   ⏪️ `revert:` (Reverting previous changes)
    *   (If multiple types apply, choose the most significant or use a generic subject).
2.  **Concise Subject Line:** After the emoji and space, write a short, imperative subject line summarizing the core change (e.g., ✨ Add user login endpoint, 🐛 Fix off-by-one error in pagination). Aim for ~50-72 characters total for the subject line.
3.  **Informative Body (Optional but Recommended):** If the changes aren't trivial, add a blank line after the subject and write a brief paragraph or bullet points explaining:
    *   *Why* the change was needed (context, problem solved).
    *   *What* the key changes are at a high level within this chunk.
4.  **Readability & Tone:** Make the message "pretty" - well-formatted, easy to scan, and professional yet approachable. Avoid unnecessary jargon.
5.  **Conciseness:** Summarize effectively; don't just list file paths unless essential for context. Keep it digestible.
"#,
        ),
        role: String::from(
            "Analyze the following `git diff` chunk and generate a high-quality commit message summarizing the changes within it.",
        ),
        example_input: Some(String::from(
            r#"diff --git a/src/main.rs b/src/main.rs
        index 1234567..89abcdef 100644
        --- a/src/main.rs
        +++ b/src/main.rs
        @@ -1,3 +1,4 @@
        +// Added a new feature
        fn main() {
        println!("Hello, world!"); }"#,
        )),
        example_output: Some(String::from("*   ✨ feat: Add new feature to main.rs\n")),
        bad_example_output: Some(String::from(
            "For the provided diff this is commit message: \n\n*   ✨ feat: Add new feature to main.rs\n",
        )),
        input: msg,
    };

    let prompt = create_prompt(&params, format.to_string().as_str())?;
    // println!("{}", prompt);

    Ok(prompt)
}

// let base_prompt_template = r#"
// **Your Role:** You are an expert developer specializing in crafting clear, concise, and engaging commit messages based on `git diff` output.

// **Your Task:** Analyze the following `git diff` chunk and generate a high-quality commit message summarizing the changes within it.

// **Commit Message Requirements:**

// 1.  **Emoji Prefix:** Start the subject line with an appropriate emoji to signify the primary nature of the change in *this chunk*:
//     *   ✨ `feat:` (New Feature)
//     *   🐛 `fix:` (Bug Fix)
//     *   ♻️ `refactor:` (Code Refactoring without behavior change)
//     *   ⚡️ `perf:` (Performance Improvement)
//     *   📝 `docs:` (Documentation updates)
//     *   🎨 `style:` (Code Style/Formatting changes)
//     *   🧪 `test:` (Adding or updating tests)
//     *   🏗️ `build:` (Build system, CI, or dependency changes)
//     *   🚀 `chore:` (Routine maintenance, tooling, or other non-code changes)
//     *   ⏪️ `revert:` (Reverting previous changes)
//     *   (If multiple types apply, choose the most significant or use a generic subject).
// 2.  **Concise Subject Line:** After the emoji and space, write a short, imperative subject line summarizing the core change (e.g., ✨ Add user login endpoint, 🐛 Fix off-by-one error in pagination). Aim for ~50-72 characters total for the subject line.
// 3.  **Informative Body (Optional but Recommended):** If the changes aren't trivial, add a blank line after the subject and write a brief paragraph or bullet points explaining:
//     *   *Why* the change was needed (context, problem solved).
//     *   *What* the key changes are at a high level within this chunk.
// 4.  **Readability & Tone:** Make the message "pretty" - well-formatted, easy to scan, and professional yet approachable. Avoid unnecessary jargon.
// 5.  **Conciseness:** Summarize effectively; don't just list file paths unless essential for context. Keep it digestible.

// **Important Note:** The provided diff might be a *part* of a larger set of changes if the original diff was split. Focus your summary on the specific changes presented in *this chunk*.

// **Input (`git diff` chunk):**
// diff {}

// **Output:**
// Generate *only* the commit message based on the provided diff chunk and the requirements above.
// "#;
