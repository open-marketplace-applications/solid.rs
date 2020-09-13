use serde::{Serialize, Deserialize};

const DEFAULT_PORT: u16 = 8080;

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub port: u16
}

impl Configuration {
    pub fn new() -> Self {
        Self { port: DEFAULT_PORT }
    }

    pub fn from_yaml(yaml_string: String) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(&yaml_string)
    }

    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }
}