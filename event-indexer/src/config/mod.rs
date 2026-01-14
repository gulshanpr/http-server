use serde::Deserialize;
use std::fs::File;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub chain: ChainConfig,
    pub contracts: Vec<ContractConfig>
}

#[derive(Debug, Deserialize)]
pub struct ChainConfig {
    pub rpc_url: String,
    pub confirmations: u64 
}

#[derive(Debug, Deserialize)]
pub struct ContractConfig {
    pub name: String,
    pub address: String,
    pub abi_path: String,
    pub events: Vec<String>
}

pub fn load_config(path: &str) -> Result<AppConfig, Box<dyn Error>> {
    let file = File::open(path)?;
    let config: AppConfig = serde_yaml::from_reader(file)?;

    Ok(config)
}