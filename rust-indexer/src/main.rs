use axum::{Router, extract::Path, response::Json, routing::get};
use axum::{extract::State, http::StatusCode};
use dotenvy::dotenv;
use ethers::providers::{Http, Provider};
use ethers::types::Block;
use ethers::types::Transaction;
use ethers::types::{Filter, H256, U256};
use ethers::utils::hex;
use ethers::{providers::Middleware, types::spoof::state};
use serde::Serialize;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::error::Error;
use std::sync::Arc;
use std::{env, u64};
use tokio::net::TcpListener;
use tokio::time::{Duration, sleep};

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
            "status": "ok"
    }))
}

async fn get_block_transactions(
    provider: &Provider<Http>,
    block_number: u64,
) -> Result<Block<Transaction>, Box<dyn Error>> {
    let block_data = provider
        .get_block_with_txs(block_number)
        .await?
        .ok_or("Block not found son")?;

    Ok(block_data)
}

// from db
async fn get_latest_transaction(
    State(state): State<AppState>,
) -> Result<Json<Vec<IndexedTransaction>>, StatusCode> {
    let rows = sqlx::query_as!(
        IndexedTransaction,
        r#"
        SELECT
            hash,
            from_address AS "from!",
            to_address AS "to",
            value,
            block_number AS "block_number!"
        FROM transactions
        ORDER BY block_number DESC
        LIMIT 10
        "#
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        eprintln!("DB error fetching latest transactions: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(rows))
}

async fn get_transaction_by_address(
    Path(address): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Vec<IndexedTransaction>>, StatusCode> {
    let result = sqlx::query_as!(
        IndexedTransaction,
        r#"
        SELECT
            hash,
            from_address as "from",
            to_address as "to",
            value,
            block_number as "block_number!"
        FROM transactions
        WHERE from_address = $1
           OR to_address = $1
        ORDER BY block_number DESC
        LIMIT 20
        "#,
        address
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|err| {
        eprint!("DB error wen getting tnx by address {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(result))
}

async fn get_erc20_transaction_by_address(
    Path(address): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Vec<ERC20Struct>>, StatusCode> {
    let results = sqlx::query_as!(
        ERC20Struct,
        r#"
        SELECT
            tx_hash      as "tnx_hash",
            log_index,
            token_address,
            from_address as "from",
            to_address   as "to",
            value,
            block_number as "block_number!"
        FROM erc20_transfers
        WHERE from_address = $1
           OR to_address   = $1
        ORDER BY block_number DESC
        LIMIT 20
        "#,
        address
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|err| {
        eprintln!("gettting error in erc20 log by address {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(results))
}

async fn get_latest_block(provider: &Provider<Http>) -> Result<u64, Box<dyn Error>> {
    let fetch_block_number = provider.get_block_number().await?;
    Ok(fetch_block_number.as_u64())
}

async fn create_db_pool() -> Result<PgPool, Box<dyn Error>> {
    let db_url = env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    Ok(pool)
}

async fn insert_tnx(pool: &PgPool, tnx: IndexedTransaction) -> Result<(), Box<dyn Error>> {
    sqlx::query!(
        r#"
        INSERT INTO transactions (hash, from_address, to_address, value, block_number)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (hash) DO NOTHING
        "#,
        tnx.hash,
        tnx.from,
        tnx.to,
        tnx.value,
        tnx.block_number as i64
    )
    .execute(pool)
    .await?;

    println!("inserted {} into db", tnx.block_number);
    Ok(())
}

async fn insert_tnx_batch(
    pool: &PgPool,
    tnxs: &[IndexedTransaction],
) -> Result<(), Box<dyn Error>> {
    // start the connection
    let mut connection = pool.begin().await?;

    // execute

    for tx in tnxs {
        sqlx::query!(
            r#"
            INSERT INTO transactions (
                hash,
                from_address,
                to_address,
                value,
                block_number
            )
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (hash) DO NOTHING
            "#,
            tx.hash,
            tx.from,
            tx.to,
            tx.value,
            tx.block_number as i64
        )
        .execute(&mut *connection)
        .await?;
    }
    // commit
    connection.commit().await?;

    Ok(())
}

async fn get_last_indexed_block_from_db(pool: &PgPool) -> Result<u64, Box<dyn Error>> {
    let block = sqlx::query!(r#"SELECT last_indexed_block FROM indexer_state WHERE id = 1"#)
        .fetch_one(pool)
        .await?;

    Ok(block.last_indexed_block as u64)
}

async fn update_indexer_state(
    pool: &PgPool,
    block_number: u64,
    block_hash: &str,
) -> Result<(), Box<dyn Error>> {
    sqlx::query!(
        r#"
        UPDATE indexer_state
        SET last_indexed_block = $1,
            last_indexed_block_hash = $2
        WHERE id = 1
        "#,
        block_number as i64,
        block_hash
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn get_last_indexed_block_hash(pool: &PgPool) -> Result<String, Box<dyn Error>> {
    let result = sqlx::query!(r#"SELECT last_indexed_block_hash FROM indexer_state WHERE id = 1"#)
        .fetch_one(pool)
        .await?;

    Ok(result.last_indexed_block_hash.unwrap_or_default())
}

async fn get_async_erc20_logs(
    provider: &Provider<Http>,
    block_number: u64,
) -> Result<Vec<ethers::types::Log>, Box<dyn Error>> {
    let transfer_topic = H256::from_slice(
        // cast keccak "Transfer(address,address,uint256)"
        &hex::decode("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef")?,
    );

    let filters = Filter::new()
        .from_block(block_number)
        .to_block(block_number)
        .topic0(transfer_topic);

    let logs = provider.get_logs(&filters).await?;

    Ok(logs)
}

fn decode_erc20_log(logs: &ethers::types::Log) -> Option<ERC20Struct> {
    if logs.topics.len() < 3 {
        return None;
    }

    let tnx_hash = logs.transaction_hash?.to_string();
    let log_index = logs.log_index?.as_u64() as i32;
    let block_number = logs.block_number?.as_u64() as i64;

    let token_address = logs.address.to_string();
    let from = H256::from(logs.topics[1]).to_string();
    let to = H256::from(logs.topics[2]).to_string();
    let value = U256::from_big_endian(&logs.data.0).to_string();

    Some(ERC20Struct {
        tnx_hash,
        log_index,
        token_address,
        from,
        to,
        value,
        block_number,
    })
}

async fn insert_erc20_into_db(pool: &PgPool, transfer: &ERC20Struct) -> Result<(), Box<dyn Error>> {
    sqlx::query!(
        r#"
        INSERT INTO erc20_transfers (
            tx_hash,
            log_index,
            token_address,
            from_address,
            to_address,
            value,
            block_number
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (tx_hash, log_index) DO NOTHING
        "#,
        transfer.tnx_hash,
        transfer.log_index,
        transfer.token_address,
        transfer.from,
        transfer.to,
        transfer.value,
        transfer.block_number as i64
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn insert_erc20_into_db_batch(
    pool: &PgPool,
    logs: &[ERC20Struct],
) -> Result<(), Box<dyn Error>> {
    // start connection
    let mut connection = pool.begin().await?;
    // execute

    for transfer_log in logs {
        sqlx::query!(
            r#"
        INSERT INTO erc20_transfers (
            tx_hash,
            log_index,
            token_address,
            from_address,
            to_address,
            value,
            block_number
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (tx_hash, log_index) DO NOTHING
        "#,
            transfer_log.tnx_hash,
            transfer_log.log_index,
            transfer_log.token_address,
            transfer_log.from,
            transfer_log.to,
            transfer_log.value,
            transfer_log.block_number as i64
        )
        .execute(&mut *connection)
        .await?;
    }

    // commit
    connection.commit().await?;

    Ok(())
}

async fn delete_transactions_by_block(
    pool: &PgPool,
    block_number: u64,
) -> Result<(), Box<dyn Error>> {
    sqlx::query!(
        r#"DELETE FROM transactions WHERE block_number = $1"#,
        block_number as i64
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn delete_erc20_by_block(pool: &PgPool, block_number: u64) -> Result<(), Box<dyn Error>> {
    sqlx::query!(
        r#"DELETE FROM erc20_transfers WHERE block_number = $1"#,
        block_number as i64
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn rewind_indexer_state(
    pool: &PgPool,
    new_block_number: u64,
    new_block_hash: &str,
) -> Result<(), Box<dyn Error>> {
    sqlx::query!(
        r#"
        UPDATE indexer_state
        SET last_indexed_block = $1,
            last_indexed_block_hash = $2
        WHERE id = 1
        "#,
        new_block_number as i64,
        new_block_hash
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[derive(Debug, Serialize)]
struct IndexedTransaction {
    hash: String,
    from: String,
    to: Option<String>,
    value: String,
    block_number: i64,
}

#[derive(Debug, Serialize)]
struct ERC20Struct {
    tnx_hash: String,
    log_index: i32,
    token_address: String,
    from: String,
    to: String,
    value: String,
    block_number: i64,
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let pool = create_db_pool().await?;

    let app_state = AppState { pool: pool.clone() };

    let apis = Router::new()
        .route("/health", get(health))
        .route("/transactions/latest", get(get_latest_transaction))
        .route(
            "/transactions/address/:address",
            get(get_transaction_by_address),
        )
        .route(
            "/erc20/transfers/address/:address",
            get(get_erc20_transaction_by_address),
        )
        .with_state(app_state);

    tokio::spawn(async move {
        match TcpListener::bind("0.0.0.0:3000").await {
            Ok(listener) => {
                if let Err(e) = axum::serve(listener, apis).await {
                    eprintln!("Server error: {}", e);
                }
            }
            Err(e) => eprintln!("Failed to bind listener: {}", e),
        }
    });

    let mut current_block_number = get_last_indexed_block_from_db(&pool).await?;
    let mut prev_block_hash = get_last_indexed_block_hash(&pool).await?;

    let max_block_per_index = 50;

    println!("Resuming from block {}", current_block_number);

    let rpc_url = env::var("ARBITURM_RPC_URL")?;
    let provider = Provider::<Http>::try_from(rpc_url)?;

    loop {
        let latest_chain_block = get_latest_block(&provider).await?;

        if current_block_number >= latest_chain_block {
            println!("No new block. Sleeping...");
            sleep(Duration::from_secs(2)).await;
            continue;
        }

        let target_block = std::cmp::min(
            current_block_number + max_block_per_index,
            latest_chain_block,
        );

        while current_block_number < target_block {
            println!("behind by {}", latest_chain_block - current_block_number);

            let next_block = current_block_number + 1;
            println!("Indexing block {}", next_block);

            let current_block = get_block_transactions(&provider, next_block).await?;

            let parent_hash = current_block.parent_hash.to_string();

            let is_bootstrap = prev_block_hash.is_empty();

            if !is_bootstrap && parent_hash != prev_block_hash {
                eprintln!("Reorg detected at block {}", next_block);

                // rollback the LAST indexed block
                delete_transactions_by_block(&pool, current_block_number).await?;
                delete_erc20_by_block(&pool, current_block_number).await?;

                current_block_number -= 1;

                // reset hash anchor (will be re-initialized)
                update_indexer_state(&pool, current_block_number, "").await?;
                prev_block_hash.clear();

                continue;
            }

            let mut log_batch = Vec::<ERC20Struct>::new();

            let logs = get_async_erc20_logs(&provider, next_block).await?;
            for log in logs {
                if let Some(transfer) = decode_erc20_log(&log) {
                    // insert_erc20_into_db(&pool, &transfer).await?;
                    log_batch.push(transfer);
                }
            }
            insert_erc20_into_db_batch(&pool, &log_batch).await?;

            let mut tnx_batch = Vec::new();

            for tx in current_block.transactions {
                let block_number = tx
                    .block_number
                    .map(|n| n.as_u64() as i64)
                    .unwrap_or(next_block as i64);

                tnx_batch.push(IndexedTransaction {
                    hash: format!("{:?}", tx.hash),
                    from: format!("{:?}", tx.from),
                    to: tx.to.map(|a| a.to_string()),
                    value: tx.value.to_string(),
                    block_number,
                });
            }

            insert_tnx_batch(&pool, &tnx_batch).await?;

            let current_block_hash = current_block.hash.ok_or("Block hash missing")?.to_string();

            update_indexer_state(&pool, next_block, &current_block_hash).await?;

            current_block_number = next_block;
            prev_block_hash = current_block_hash;
        }
    }
}
