use std::fmt;

#[derive(Debug, Clone)]
pub struct Agent {
    pub name: Option<String>,
    pub description: Option<String>,
    pub agent_do: Vec<String>,
    pub agent_dont: Vec<String>,
}

impl Agent {
    pub fn new() -> Self {
        Agent {
            name: None,
            description: None,
            agent_do: Vec::new(),
            agent_dont: Vec::new(),
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_agent_do(mut self, do_action: impl Into<String>) -> Self {
        self.agent_do.push(do_action.into());
        self
    }

    pub fn with_agent_dont(mut self, dont_action: impl Into<String>) -> Self {
        self.agent_dont.push(dont_action.into());
        self
    }

    // Helper to add multiple do actions at once
    pub fn with_agent_dos(mut self, do_actions: Vec<impl Into<String>>) -> Self {
        for action in do_actions {
            self.agent_do.push(action.into());
        }
        self
    }

    // Helper to add multiple don't actions at once
    pub fn with_agent_donts(mut self, dont_actions: Vec<impl Into<String>>) -> Self {
        for action in dont_actions {
            self.agent_dont.push(action.into());
        }
        self
    }

    // Merge with another agent
    pub fn merge(&self, other: &Agent) -> Self {
        let mut result = self.clone();

        if result.name.is_none() {
            result.name = other.name.clone();
        }

        if result.description.is_none() {
            result.description = other.description.clone();
        }

        for do_action in &other.agent_do {
            if !result.agent_do.contains(do_action) {
                result.agent_do.push(do_action.clone());
            }
        }

        for dont_action in &other.agent_dont {
            if !result.agent_dont.contains(dont_action) {
                result.agent_dont.push(dont_action.clone());
            }
        }

        result
    }
}

impl fmt::Display for Agent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.name {
            writeln!(f, "# Agent: {}", name)?;
        }

        if let Some(description) = &self.description {
            writeln!(f, "\n## Description\n\n{}", description)?;
        }

        if !self.agent_do.is_empty() {
            writeln!(f, "\n## Agent Should")?;
            for do_action in &self.agent_do {
                writeln!(f, "- {}", do_action)?;
            }
        }

        if !self.agent_dont.is_empty() {
            writeln!(f, "\n## Agent Should Not")?;
            for dont_action in &self.agent_dont {
                writeln!(f, "- {}", dont_action)?;
            }
        }

        Ok(())
    }
}

impl Default for Agent {
    fn default() -> Self {
        Self::new()
    }
}
