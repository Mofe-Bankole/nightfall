use anyhow::Context;

pub struct NightfallAPIClient;

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
