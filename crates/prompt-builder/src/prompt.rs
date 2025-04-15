use crate::agent::Agent;
use crate::formatters::{CustomFormatter, Format, Formattable};
use crate::project::Project;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum StepType {
    INSTRUCTION,
    RUN,
    ASK,
    THINK,
    ACT,
    TEST,
    CUSTOM(String),
}

impl StepType {
    pub fn as_str(&self) -> &str {
        match self {
            StepType::INSTRUCTION => "instruction",
            StepType::RUN => "run",
            StepType::ASK => "ask",
            StepType::THINK => "think",
            StepType::ACT => "act",
            StepType::TEST => "test",
            StepType::CUSTOM(s) => s,
        }
    }
}

impl From<&str> for StepType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "instruction" => StepType::INSTRUCTION,
            "run" => StepType::RUN,
            "ask" => StepType::ASK,
            "think" => StepType::THINK,
            "act" => StepType::ACT,
            "test" => StepType::TEST,
            _ => StepType::CUSTOM(s.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Step {
    step_type: StepType,
    text: String,
    continue_on_error: bool,
    action_on_error: Option<String>,
    action_on_success: Option<String>,
}

impl Step {
    pub fn new(step_type: StepType, text: impl Into<String>) -> Self {
        Step {
            step_type,
            text: text.into(),
            continue_on_error: false,
            action_on_error: None,
            action_on_success: None,
        }
    }

    pub fn builder() -> StepBuilder {
        StepBuilder::new()
    }

    pub fn with_continue_on_error(mut self, continue_on_error: bool) -> Self {
        self.continue_on_error = continue_on_error;
        self
    }

    pub fn with_action_on_error(mut self, action: impl Into<String>) -> Self {
        self.action_on_error = Some(action.into());
        self
    }

    pub fn with_action_on_success(mut self, action: impl Into<String>) -> Self {
        self.action_on_success = Some(action.into());
        self
    }
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "**{}**: {}", self.step_type.as_str(), self.text)?;

        if self.continue_on_error {
            write!(f, " (continue on error)")?;
        }

        if let Some(action) = &self.action_on_error {
            write!(f, "\n  - On error: {}", action)?;
        }

        if let Some(action) = &self.action_on_success {
            write!(f, "\n  - On success: {}", action)?;
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct StepBuilder {
    step_type: Option<StepType>,
    text: Option<String>,
    continue_on_error: bool,
    action_on_error: Option<String>,
    action_on_success: Option<String>,
}

impl StepBuilder {
    pub fn new() -> Self {
        StepBuilder {
            step_type: None,
            text: None,
            continue_on_error: false,
            action_on_error: None,
            action_on_success: None,
        }
    }

    pub fn with_type(mut self, step_type: StepType) -> Self {
        self.step_type = Some(step_type);
        self
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn with_continue_on_error(mut self, continue_on_error: bool) -> Self {
        self.continue_on_error = continue_on_error;
        self
    }

    pub fn with_action_on_error(mut self, action: impl Into<String>) -> Self {
        self.action_on_error = Some(action.into());
        self
    }

    pub fn with_action_on_success(mut self, action: impl Into<String>) -> Self {
        self.action_on_success = Some(action.into());
        self
    }

    pub fn build(self) -> Step {
        Step {
            step_type: self.step_type.unwrap_or(StepType::INSTRUCTION),
            text: self.text.unwrap_or_default(),
            continue_on_error: self.continue_on_error,
            action_on_error: self.action_on_error,
            action_on_success: self.action_on_success,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Prompt {
    pub agent: Option<Agent>,
    pub project: Option<Project>,
    pub goal: Option<String>,
    pub steps: Vec<Step>,
    pub rules: Vec<String>,
}

impl Prompt {
    // Step type constants for convenience
    pub const INSTRUCTION: StepType = StepType::INSTRUCTION;
    pub const RUN: StepType = StepType::RUN;
    pub const ASK: StepType = StepType::ASK;
    pub const THINK: StepType = StepType::THINK;
    pub const ACT: StepType = StepType::ACT;
    pub const TEST: StepType = StepType::TEST;

    pub fn new() -> Self {
        Prompt {
            agent: None,
            project: None,
            goal: None,
            steps: Vec::new(),
            rules: Vec::new(),
        }
    }

    pub fn with_agent(mut self, agent: &Agent) -> Self {
        self.agent = Some(agent.clone());
        self
    }

    pub fn with_project(mut self, project: &Project) -> Self {
        self.project = Some(project.clone());
        self
    }

    pub fn with_goal(mut self, goal: impl Into<String>) -> Self {
        self.goal = Some(goal.into());
        self
    }

    pub fn with_steps(mut self, steps: Vec<Step>) -> Self {
        self.steps = steps;
        self
    }

    pub fn add_step(mut self, step_type: impl Into<StepType>, text: impl Into<String>) -> Self {
        self.steps.push(Step::new(step_type.into(), text.into()));
        self
    }

    pub fn add_step_obj(mut self, step: Step) -> Self {
        self.steps.push(step);
        self
    }

    pub fn with_rule(mut self, rule: impl Into<String>) -> Self {
        self.rules.push(rule.into());
        self
    }

    pub fn with_rules(mut self, rules: Vec<impl Into<String>>) -> Self {
        for rule in rules {
            self.rules.push(rule.into());
        }
        self
    }

    // Merge with another prompt
    pub fn merge(&self, other: &Prompt) -> Self {
        let mut result = self.clone();

        if result.agent.is_none() {
            result.agent = other.agent.clone();
        } else if let Some(other_agent) = &other.agent {
            if let Some(agent) = &result.agent {
                result.agent = Some(agent.merge(other_agent));
            }
        }

        if result.project.is_none() {
            result.project = other.project.clone();
        } else if let Some(other_project) = &other.project {
            if let Some(project) = &result.project {
                result.project = Some(project.merge(other_project));
            }
        }

        if result.goal.is_none() {
            result.goal = other.goal.clone();
        }

        for step in &other.steps {
            result.steps.push(step.clone());
        }

        for rule in &other.rules {
            if !result.rules.contains(rule) {
                result.rules.push(rule.clone());
            }
        }

        result
    }

    // Build the final prompt
    pub fn build(self) -> Self {
        self
    }

    // Convert to a specific format
    pub fn to_format(&self, format: Format) -> String {
        match format {
            Format::Markdown => self.to_string(),
            Format::Xml => self.to_xml(),
            Format::SemiXml => self.to_semi_xml(),
        }
    }

    // Convert to XML format
    fn to_xml(&self) -> String {
        let mut result = String::new();

        result.push_str("<prompt>\n");

        if let Some(agent) = &self.agent {
            if let Some(name) = &agent.name {
                result.push_str(&format!("  <agent_name>{}</agent_name>\n", name));
            }

            // Add other agent properties...
        }

        if let Some(project) = &self.project {
            if let Some(name) = &project.name {
                result.push_str(&format!("  <project_name>{}</project_name>\n", name));
            }

            // Add other project properties...
        }

        if let Some(goal) = &self.goal {
            result.push_str(&format!("  <goal>{}</goal>\n", goal));
        }

        if !self.steps.is_empty() {
            result.push_str("  <steps>\n");
            for (i, step) in self.steps.iter().enumerate() {
                result.push_str(&format!(
                    "    <step number=\"{}\" type=\"{}\">{}</step>\n",
                    i + 1,
                    step.step_type.as_str(),
                    step.text
                ));
            }
            result.push_str("  </steps>\n");
        }

        if !self.rules.is_empty() {
            result.push_str("  <rules>\n");
            for rule in &self.rules {
                result.push_str(&format!("    <rule>{}</rule>\n", rule));
            }
            result.push_str("  </rules>\n");
        }

        result.push_str("</prompt>");

        result
    }

    // Convert to Semi-XML format (mix of XML and Markdown)
    fn to_semi_xml(&self) -> String {
        let mut result = String::new();

        if let Some(agent) = &self.agent {
            if let Some(name) = &agent.name {
                result.push_str(&format!("# <agent_name>{}</agent_name>\n\n", name));
            }

            if let Some(description) = &agent.description {
                result.push_str(&format!("## Description\n\n{}\n\n", description));
            }
        }

        if let Some(project) = &self.project {
            if let Some(name) = &project.name {
                result.push_str(&format!("# <project_name>{}</project_name>\n\n", name));
            }

            if let Some(description) = &project.description {
                result.push_str(&format!("## Description\n\n{}\n\n", description));
            }
        }

        if let Some(goal) = &self.goal {
            result.push_str(&format!("## <goal>{}</goal>\n\n", goal));
        }

        if !self.steps.is_empty() {
            result.push_str("## <steps>\n\n");
            for (i, step) in self.steps.iter().enumerate() {
                result.push_str(&format!(
                    "{}. **{}**: {}\n",
                    i + 1,
                    step.step_type.as_str(),
                    step.text
                ));
            }
            result.push_str("\n</steps>\n\n");
        }

        if !self.rules.is_empty() {
            result.push_str("## <rules>\n\n");
            for rule in &self.rules {
                result.push_str(&format!("- {}\n", rule));
            }
            result.push_str("\n</rules>\n");
        }

        result
    }

    // Convert to a custom format using a template
    pub fn to_custom_format(&self, formatter: &CustomFormatter) -> String {
        formatter.format(self)
    }
}

impl fmt::Display for Prompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(agent) = &self.agent {
            write!(f, "{}\n\n", agent)?;
        }

        if let Some(project) = &self.project {
            write!(f, "{}\n\n", project)?;
        }

        if let Some(goal) = &self.goal {
            writeln!(f, "## Goal\n\n{}\n", goal)?;
        }

        if !self.steps.is_empty() {
            writeln!(f, "## Steps")?;
            for (i, step) in self.steps.iter().enumerate() {
                writeln!(f, "{}. {}", i + 1, step)?;
            }
            writeln!(f)?;
        }

        if !self.rules.is_empty() {
            writeln!(f, "## Rules")?;
            for rule in &self.rules {
                writeln!(f, "- {}", rule)?;
            }
        }

        Ok(())
    }
}

impl Default for Prompt {
    fn default() -> Self {
        Self::new()
    }
}

impl Formattable for Prompt {}
