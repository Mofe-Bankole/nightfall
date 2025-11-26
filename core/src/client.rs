use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json::Value;
pub struct NightfallAPIClient;
#[derive(Debug , Serialize , Deserialize)]
pub struct BlockchainStats{
    pub blockhain_height : u64,
    pub latest_block_time : u64,
}
pub struct Stats{
    pub blockhain_height : u32,
}
pub async fn get_latest_block_height_raw() -> anyhow::Result<reqwest::Response> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://api.testnet.cipherscan.app/api/blocks?limit=1&offset=0")
        .send()
        .await
        .context("Failed to connect to cipherscan API")?;

    Ok(response)
}

pub async fn get_transaction_by_id(txid: &str) -> anyhow::Result<reqwest::Response> {
    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();

    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );

    let url = format!("https://api.testnet.cipherscan.app/api/tx/{}", txid);
    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .context("Failed to fetch transaction by id")?;

    Ok(response)
}

pub async fn fetch_latest_block_height() -> anyhow::Result<reqwest::Response> {
    let client = reqwest::Client::new();

    let url = "https://api.testnet.cipherscan.app/api/network/stats";

    let response = client
        .get(url)
        .send()
        .await?
        .json::<BlockchainStats>()
        .await?;

    // let height = response["blockchain"]["height"];
    // let parsed_height = height.await;
    Ok(response)
} 
