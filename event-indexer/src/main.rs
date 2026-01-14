mod config;
mod types;

use crate::types::{decode_log, load_abi};
use config::load_config;
use ethers::prelude::Http;
use ethers::providers::{Middleware, Provider};
use ethers::types::{Address, BlockNumber, Filter};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = load_config("config.yaml")?;

    for contract in &config.contracts {
        println!("Loading contract abi {}", contract.name);

        let abi = load_abi(&contract.abi_path)?;

        println!(
            "ABI load for contract {} with total function count {}",
            contract.name,
            abi.functions().count()
        );
    }

    let provider = Provider::<Http>::try_from(&config.chain.rpc_url)?;

    let contract = &config.contracts[0];

    let abi = load_abi(&contract.abi_path)?;

    let address: Address = contract.address.parse()?;

    let filters = Filter::new()
    .address(address)
    .from_block(BlockNumber::Latest);

    let logs = provider.get_logs(&filters).await?;
    println!("logs fetched {}", logs.len());

    for log in logs {
        if let Some(decoded) = decode_log(&log, &abi, &contract.name) {
            println!("Contract: {}", decoded.contract);
            println!("Event: {}", decoded.event);
            println!("Block: {}", decoded.block_number);
            println!("{:#?}", decoded.fields);

            break; // stop after first decoded event
        }
    }

    // normal way if ? is not used and ? is only for Result and Option
    // let file_data = match file {
    //     Ok(data) => data,
    //     Err(err) => return Err(err)
    // };

    Ok(())
}
