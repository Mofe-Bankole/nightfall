use std::{collections::HashMap, env};
pub async fn create_client() -> Result<(), anyhow::Error> {
    let rpc_url = env::var("RPC_URL")
        .unwrap_or_else(|_| "https://api.testnet.cipherscan.app/api".to_string());

    let client = reqwest::get(rpc_url)
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    println!("{client:#?}");
    Ok(())
}
