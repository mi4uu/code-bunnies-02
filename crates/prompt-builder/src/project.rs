use std::fmt;

#[derive(Debug, Clone)]
pub struct Project {
    pub name: Option<String>,
    pub description: Option<String>,
    pub languages: Vec<String>,
    pub use_libs: Vec<String>,
    pub dont_use: Vec<String>,
    pub requirements: Vec<String>,
    pub rules: Vec<String>,
    pub good_examples: Vec<String>,
    pub bad_examples: Vec<String>,
}

impl Project {
    // Common language constants
    pub const WITH_RUST: &'static str = "rust";
    pub const WITH_GOLANG: &'static str = "golang";
    pub const WITH_JAVASCRIPT: &'static str = "javascript";
    pub const WITH_TYPESCRIPT: &'static str = "typescript";
    pub const WITH_PYTHON: &'static str = "python";
    pub const WITH_CSS: &'static str = "css";
    pub const WITH_HTML: &'static str = "html";

    pub fn new() -> Self {
        Project {
            name: None,
            description: None,
            languages: Vec::new(),
            use_libs: Vec::new(),
            dont_use: Vec::new(),
            requirements: Vec::new(),
            rules: Vec::new(),
            good_examples: Vec::new(),
            bad_examples: Vec::new(),
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

    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.languages.push(language.into());
        self
    }

    pub fn with_languages(mut self, languages: Vec<impl Into<String>>) -> Self {
        for lang in languages {
            self.languages.push(lang.into());
        }
        self
    }

    pub fn with_use(mut self, lib: impl Into<String>) -> Self {
        self.use_libs.push(lib.into());
        self
    }

    pub fn with_uses(mut self, libs: Vec<impl Into<String>>) -> Self {
        for lib in libs {
            self.use_libs.push(lib.into());
        }
        self
    }

    pub fn with_dont_use(mut self, lib: impl Into<String>) -> Self {
        self.dont_use.push(lib.into());
        self
    }

    pub fn with_requirement(mut self, requirement: impl Into<String>) -> Self {
        self.requirements.push(requirement.into());
        self
    }

    pub fn with_rule(mut self, rule: impl Into<String>) -> Self {
        self.rules.push(rule.into());
        self
    }

    pub fn with_good_example(mut self, example: impl Into<String>) -> Self {
        self.good_examples.push(example.into());
        self
    }

    pub fn with_bad_example(mut self, example: impl Into<String>) -> Self {
        self.bad_examples.push(example.into());
        self
    }

    // Merge with another project
    pub fn merge(&self, other: &Project) -> Self {
        let mut result = self.clone();

        if result.name.is_none() {
            result.name = other.name.clone();
        }

        if result.description.is_none() {
            result.description = other.description.clone();
        }

        for lang in &other.languages {
            if !result.languages.contains(lang) {
                result.languages.push(lang.clone());
            }
        }

        for lib in &other.use_libs {
            if !result.use_libs.contains(lib) {
                result.use_libs.push(lib.clone());
            }
        }

        for lib in &other.dont_use {
            if !result.dont_use.contains(lib) {
                result.dont_use.push(lib.clone());
            }
        }

        for req in &other.requirements {
            if !result.requirements.contains(req) {
                result.requirements.push(req.clone());
            }
        }

        for rule in &other.rules {
            if !result.rules.contains(rule) {
                result.rules.push(rule.clone());
            }
        }

        for example in &other.good_examples {
            if !result.good_examples.contains(example) {
                result.good_examples.push(example.clone());
            }
        }

        for example in &other.bad_examples {
            if !result.bad_examples.contains(example) {
                result.bad_examples.push(example.clone());
            }
        }

        result
    }
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.name {
            writeln!(f, "# Project: {}", name)?;
        }

        if let Some(description) = &self.description {
            writeln!(f, "\n## Description\n\n{}", description)?;
        }

        if !self.languages.is_empty() {
            writeln!(f, "\n## Languages")?;
            for lang in &self.languages {
                writeln!(f, "- {}", lang)?;
            }
        }

        if !self.use_libs.is_empty() {
            writeln!(f, "\n## Use Libraries/Packages")?;
            for lib in &self.use_libs {
                writeln!(f, "- {}", lib)?;
            }
        }

        if !self.dont_use.is_empty() {
            writeln!(f, "\n## Don't Use")?;
            for lib in &self.dont_use {
                writeln!(f, "- {}", lib)?;
            }
        }

        if !self.requirements.is_empty() {
            writeln!(f, "\n## Requirements")?;
            for req in &self.requirements {
                writeln!(f, "- {}", req)?;
            }
        }

        if !self.rules.is_empty() {
            writeln!(f, "\n## Rules")?;
            for rule in &self.rules {
                writeln!(f, "- {}", rule)?;
            }
        }

        if !self.good_examples.is_empty() {
            writeln!(f, "\n## Good Examples")?;
            for example in &self.good_examples {
                writeln!(f, "- {}", example)?;
            }
        }

        if !self.bad_examples.is_empty() {
            writeln!(f, "\n## Bad Examples")?;
            for example in &self.bad_examples {
                writeln!(f, "- {}", example)?;
            }
        }

        Ok(())
    }
}

impl Default for Project {
    fn default() -> Self {
        Self::new()
    }
}
