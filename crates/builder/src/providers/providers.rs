use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use super::provider::Provider;
use super::provider::ProviderType;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct Providers {
    pub providers: Vec<Provider>,
    pub default_provider: String,
}
impl Providers {
    pub fn get_config_path() -> PathBuf {
        // Set environment variable for config path if not already set
        if std::env::var("CONFIG_ROOT_DIR").is_err() {
            panic!("CONFIG_ROOT_DIR environment variable is not set.");
        }

        PathBuf::from(std::env::var("CONFIG_ROOT_DIR").unwrap()).join("providers.toml")
    }

    pub fn save(&self) {
        let cfg_path = Self::get_config_path();
        fs::create_dir_all(cfg_path.parent().unwrap()).expect("Cannot create directory.");
        toml::to_string(self)
            .map(|s| {
                let mut buffer = File::create(&cfg_path).expect("Cannot create file.");

                buffer
                    .write_all(s.as_bytes())
                    .expect("Unable to write file");
            })
            .unwrap();
    }

    pub fn load() -> Self {
        let cfg_path = Self::get_config_path();
        if cfg_path.exists() {
            let content = std::fs::read_to_string(cfg_path).expect("Unable to read file");
            toml::from_str(&content).unwrap()
        } else {
            let default = Provider::provider(ProviderType::LmStudio);
            let inst = Self {
                providers: vec![default.clone(), Provider::provider(ProviderType::Ollama)],
                default_provider: default.name.clone(),
            };
            inst.save();
            inst
        }
    }

    // Get a provider by its name
    pub fn get_by_name(&self, name: &str) -> Option<&Provider> {
        self.providers.iter().find(|provider| provider.name == name)
    }

    // Get a mutable reference to a provider by its name
    pub fn get_by_name_mut(&mut self, name: &str) -> Option<&mut Provider> {
        self.providers
            .iter_mut()
            .find(|provider| provider.name == name)
    }

    // Convenience method to get the default provider
    pub fn get_default(&self) -> Option<&Provider> {
        self.get_by_name(&self.default_provider)
    }
}
