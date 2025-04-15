use crate::prompt::Prompt;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Markdown,
    Xml,
    SemiXml,
}

pub trait Formattable {}

#[derive(Debug, Clone)]
pub struct CustomFormatter {
    template: String,
    variables: HashMap<String, String>,
}

impl CustomFormatter {
    pub fn new() -> Self {
        CustomFormatter {
            template: String::new(),
            variables: HashMap::new(),
        }
    }

    pub fn with_template(mut self, template: impl Into<String>) -> Self {
        self.template = template.into();
        self
    }

    pub fn with_variable(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.variables.insert(key.into(), value.into());
        self
    }

    pub fn format(&self, prompt: &Prompt) -> String {
        let mut result = self.template.clone();

        // Replace variables from the prompt
        if let Some(agent) = &prompt.agent {
            if let Some(name) = &agent.name {
                result = result.replace("{{agent_name}}", name);
            }
        }

        if let Some(project) = &prompt.project {
            if let Some(name) = &project.name {
                result = result.replace("{{project_name}}", name);
            }

            if let Some(description) = &project.description {
                result = result.replace("{{project_description}}", description);
            }
        }

        if let Some(goal) = &prompt.goal {
            result = result.replace("{{goal}}", goal);
        }

        // Format steps
        let steps_str = if !prompt.steps.is_empty() {
            let mut steps = String::new();
            for (i, step) in prompt.steps.iter().enumerate() {
                steps.push_str(&format!("{}. {}\n", i + 1, step));
            }
            steps
        } else {
            String::new()
        };

        result = result.replace("{{steps}}", &steps_str);

        // Replace custom variables
        for (key, value) in &self.variables {
            result = result.replace(&format!("{{{{{}}}}}", key), value);
        }

        result
    }
}

impl Default for CustomFormatter {
    fn default() -> Self {
        Self::new()
    }
}
