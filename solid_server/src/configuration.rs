use std::net::{IpAddr, Ipv4Addr};
use serde::{Serialize, Deserialize};

const DEFAULT_BIND_ADDRESS: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const DEFAULT_PORT: u16 = 8080;

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub server: ServerConfiguration
}

impl Configuration {
    pub fn new() -> Self {
        Self { 
            server: ServerConfiguration {
                bind_address: IpAddr::V4(DEFAULT_BIND_ADDRESS),
                port: DEFAULT_PORT
            }
        }
    }

    pub fn from_yaml(yaml_string: String) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(&yaml_string)
    }

    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ServerConfiguration {
    #[serde(rename = "bind-address")]
    pub bind_address: IpAddr,
    pub port: u16,
}