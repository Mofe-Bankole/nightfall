use anyhow::{Ok, anyhow};
use orchard::Anchor;
use pczt::Pczt;
use serde::{Deserialize, Serialize};
use std::path::Path;
// use zcash_client_backend::wallet::tx_builder::TransactionBuilder;
use zcash_client_backend::{
    encoding::decode_payment_address,
    keys::sapling::{ExtendedFullViewingKey, ExtendedSpendingKey},
};
use zcash_primitives::{memo::Memo, transaction::builder::Builder};
use zcash_proofs::prover::LocalTxProver;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PcztJson {
    pub id: String,
    pub network: Network,
    pub created_at: String,
    pub inputs: Inputs,
    pub outputs: Outputs,
    pub prover: Prover,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
    // Fee field is critical for transaction construction
    #[serde(default)]
    pub fee: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Network {
    Mainnet,
    Testnet,
    Regtest,
}

impl Network {
    pub fn hrp(&self) -> &'static str {
        match self {
            Network::Mainnet => "zs",
            Network::Regtest => "zregtestsapling",
            Network::Testnet => "ztestsapling",
        }
    }

    pub fn orchard_hrp(&self) -> &'static str {
        match self {
            Network::Mainnet => "u",
            Network::Regtest => "uregtest",
            Network::Testnet => "utest",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Inputs {
    #[serde(default)]
    pub transparent: Vec<TransparentInput>,
    #[serde(default)]
    pub sapling: Vec<SaplingInput>,
    #[serde(default)]
    pub orchard: Vec<OrchardInput>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransparentInput {
    pub txid: String,
    pub vout: u32,
    pub amount: u64,
    pub script_pubkey: String,
    #[serde(default)]
    pub redeem_script: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SaplingInput {
    pub note_commitment: String,
    pub witness: String,
    pub amount: u64,
    #[serde(default)]
    pub rseed: Option<String>, // Randomness seed
    #[serde(default)]
    pub diversifier: Option<String>,
}

// CRITICAL: Orchard input structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrchardInput {
    pub note_commitment: String, // Note commitment (rho)
    pub witness: String,         // Merkle path witness
    pub amount: u64,
    pub rseed: String,       // Randomness seed (required for Orchard)
    pub rho: String,         // Nullifier seed
    pub diversifier: String, // Address diversifier
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Outputs {
    #[serde(default)]
    pub sapling: Vec<SaplingOutput>, // Renamed for clarity
    #[serde(default)]
    pub orchard: Vec<OrchardOutput>, // CRITICAL: Orchard outputs
    #[serde(default)]
    pub transparent: Vec<TransparentOutput>,
    pub change: Change,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SaplingOutput {
    pub address: String,
    pub amount: u64,
    #[serde(default)]
    pub memo: Option<String>,
    #[serde(default)]
    pub memo_bytes: Option<Vec<u8>>, // Alternative: raw memo bytes
}

// CRITICAL: Orchard output structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrchardOutput {
    // Should be unified address or Orchard receiver (must begin with 'u')
    pub address: String,
    pub amount: u64,
    #[serde(default)]
    pub memo: Option<String>,
    #[serde(default)]
    pub memo_bytes: Option<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransparentOutput {
    pub address: String,
    pub amount: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Change {
    pub pool: ChangePool, // Which pool to send change to
    pub to: ChangeDestination,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ChangePool {
    Transparent,
    Sapling,
    Orchard, // CRITICAL: Support Orchard change
    Auto,    // Let the builder decide
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ChangeDestination {
    Auto(String),    // "auto"
    Address(String), // explicit address
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prover {
    #[serde(default)]
    pub spending_key: Option<String>, // Unified spending key
    #[serde(default)]
    pub sapling_spending_key: Option<String>, // Sapling-specific key
    #[serde(default)]
    pub orchard_spending_key: Option<String>, // CRITICAL: Orchard spending key
    #[serde(default)]
    pub proof_blobs: Vec<String>,
    #[serde(default)]
    pub pczt_signed_tx_hex: Option<String>,
    #[serde(default)]
    pub anchor: Option<String>, // Merkle tree anchor for notes
}

#[derive(Debug, Clone)]
pub struct UnsignedBundle {
    pub prover: Prover,
    #[serde(default)]
    pub anchor: Anchor,
}

#[derive(Debug, Clone)]
pub struct PCZTBuilder {
    pub network: Network,
    pub inputs: Inputs,
    pub outputs: Outputs,
    pub fee: u64,
}

impl PCZTBuilder {
    pub fn try_from_json(data: PcztJson) -> Result<Self, anyhow::Error> {
        // Validate that we have at least one output
        if !data.outputs.sapling.is_empty()
            && data.outputs.orchard.is_empty()
            && data.outputs.transparent.is_empty()
        {
            return Err(anyhow!("Transaction has no outputs"));
        }

        // Validate Sapling outputs
        for out in &data.outputs.sapling {
            if out.amount == 0 {
                return Err(anyhow!("Amount must be > 0"));
            }

            // Network (testnet , mainnet , rregtest)
            let hrp = data.network.hrp();
            decode_payment_address(hrp, &out.address)
                .map_err(|_| anyhow!("Invalid Sapling Address : {}", out.address))?;
        }

        // Validate Orchard outputs
        for out in &data.outputs.orchard {
            if out.amount == 0 {
                return Err(anyhow!("Amount must be > 0"));
            }

            // Orchard uses unified addresses with "u" prefix
            let orchard_hrp = data.network.orchard_hrp();
            // Note: You may need additional validation for unified addresses
            if !out.address.starts_with(orchard_hrp) {
                return Err(anyhow!(
                    "Invalid Orchard / Unified Address (expected prefix '{}'): {}",
                    orchard_hrp,
                    out.address
                ));
            }
        }

        // Validate transparent outputs
        for out in &data.outputs.transparent {
            if out.amount == 0 {
                return Err(anyhow!("Amount must be > 0"));
            }
        }

        // fee is 1000 zatoshis
        let data_fee = data.fee.expect("no zatoshis");

        Ok(Self {
            network: data.network,
            inputs: data.inputs,
            outputs: data.outputs,
            fee: data_fee,
        })
    }

    pub fn build_unsigned(
        &mut self,
        network: Network,
        height: u32,
        extsk: &ExtendedSpendingKey,
    ) -> anyhow::Result<UnsignedBundle> {
        let tx_prover = LocalTxProver::bundled();
        let extsk_key = ExtendedFullViewingKey::new(extsk);

        let mut builder = Builder::new(params, target_height, build_config);
    }
}
