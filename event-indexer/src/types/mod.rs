use ethers::abi::{Abi, RawLog};
use std::error::Error;
use std::fs::File;
use serde_json::Value;
use std::collections::HashMap;
use serde_json::json;
use ethers::types::Log;

#[derive(Debug)]
pub struct DecodedEvent {
    pub contract: String,
    pub event: String,
    pub block_number: u64,
    pub fields: HashMap<String, Value>,
}

pub fn load_abi(path: &str) -> Result<Abi, Box<dyn Error>> {
    let file = File::open(path)?;
    let abi = serde_json::from_reader(file)?;

    Ok(abi)
}

pub fn decode_log(log: &Log, abi: &Abi, contract_name: &str) -> Option<(DecodedEvent)> {
    let raw_logs = RawLog {
        topics: log.topics.clone(),
        data: log.data.to_vec(),
    };

    for event in abi.events() {
        if let Ok(decoded) = event.parse_log(raw_logs.clone()) {
            let mut fields = HashMap::new();

            for params in decoded.params {
                fields.insert(params.name, json!(params.value));
            }

            return Some(
                DecodedEvent {
                    contract: contract_name.to_string(),
                    event: event.name.clone(),
                    block_number: log.block_number.unwrap_or_default().as_u64(),
                    fields: fields,
                }
            );
        }
    }

    None
}