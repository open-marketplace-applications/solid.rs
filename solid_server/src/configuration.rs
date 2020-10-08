use std::net::{IpAddr, Ipv4Addr};
use serde::{Serialize, Deserialize};
use anyhow::Result;

const DEFAULT_BIND_ADDRESS: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const DEFAULT_PORT: u16 = 8080;

const DEFAULT_STORAGE_PROVIDER: &str = "./solid-fs";

const DEFAULT_LOG_TO_TERMINAL: bool = true;
const DEFAULT_LOG_TO_FILE: bool = false;
const DEFAULT_LOG_FILE_PATH: &str = "./solid.log";

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub server: ServerConfiguration,
    pub storage: StorageConfiguration,
    pub logging: LoggingConfiguration
}

impl Configuration {
    pub fn new() -> Self {
        Self { 
            server: ServerConfiguration {
                bind_address: IpAddr::V4(DEFAULT_BIND_ADDRESS),
                port: DEFAULT_PORT
            },
            storage: StorageConfiguration {
                storage_provider: StorageProvider::LocalFilesystem(DEFAULT_STORAGE_PROVIDER.into())
            },
            logging: LoggingConfiguration {
                log_to_terminal: DEFAULT_LOG_TO_TERMINAL,
                log_to_file: DEFAULT_LOG_TO_FILE,
                log_file_path: DEFAULT_LOG_FILE_PATH.into()
            }
        }
    }

    pub fn from_yaml(yaml_string: String) -> Result<Self> {
        Ok(serde_yaml::from_str(&yaml_string)?)
    }

    pub fn to_yaml(&self) -> Result<String> {
        Ok(serde_yaml::to_string(self)?)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ServerConfiguration {
    #[serde(rename = "bind-address")]
    pub bind_address: IpAddr,
    pub port: u16,
}

#[derive(Serialize, Deserialize)]
pub struct StorageConfiguration {
    #[serde(rename = "storage-provide")]
    pub storage_provider: StorageProvider,
}

#[derive(Serialize, Deserialize)]
pub enum StorageProvider {
    #[serde(rename = "local-filesystem")]
    LocalFilesystem(String),
    #[serde(rename = "in-memory-filesystem")]
    InMemoryFilesystem
}
#[derive(Serialize, Deserialize)]
pub struct LoggingConfiguration {
    #[serde(rename = "log-to-terminal")]
    pub log_to_terminal: bool,
    #[serde(rename = "log-to-file")]
    pub log_to_file: bool,
    #[serde(rename = "log-file-path")]
    pub log_file_path: String,
}
