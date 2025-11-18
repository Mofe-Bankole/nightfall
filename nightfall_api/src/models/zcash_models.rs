use anyhow::anyhow;
use orchard::value::VALUE_SUM_RANGE;
use pczt::Pczt;
use serde::{Deserialize, Serialize};
use zcash_client_backend::encoding::decode_payment_address;
use zcash_primitives::memo::Memo;

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
            Network::Testnet => "ztestsapling",
            Network::Regtest => "zregtestsapling",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Inputs {
    #[serde(default)]
    pub transparent: Vec<TransparentInput>,
    #[serde(default)]
    pub shielded: Vec<ShieldedInput>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransparentInput {
    pub txid: String,
    pub vout: u32,
    pub amount: u64,
    pub script_pubkey: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShieldedInput {
    pub note_commitment: String,
    pub witness: String,
    pub amount: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Outputs {
    #[serde(default)]
    pub shielded: Vec<ShieldedOutput>,
    #[serde(default)]
    pub transparent: Vec<TransparentOutput>,
    pub change: Change,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShieldedOutput {
    pub address: String,
    pub amount: u64,
    #[serde(default)]
    pub memo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransparentOutput {
    pub address: String,
    pub amount: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Change {
    pub shielded: bool,
    pub to: ChangeDestination,
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
    pub spending_key: Option<String>,
    #[serde(default)]
    pub proof_blobs: Vec<String>,
    #[serde(default)]
    pub pczt_signed_tx_hex: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PCZTBuilder {
    pub network: Network,
    pub outputs: Outputs,
    pub memo: Memo,
}

impl PCZTBuilder {
    pub fn try_from_json(data: PcztJson) -> Result<Pczt, anyhow::Error> {
        let shielded = &data.outputs.shielded;
        if shielded.is_empty() {
            return Err(anyhow!("No shielded outputs present"));
        }

        for out in shielded {
            if out.amount == 0 {
                return Err(anyhow!("Amount must be > 0"));
            }

            // Decode the shielded address for the given network.
            let hrp = data.network.hrp();
            decode_payment_address(hrp, &out.address)
                .map_err(|_| anyhow!("Invalid Shielded Address : {}", out.address))?;
        }

        let hex = &data
            .prover
            .pczt_signed_tx_hex
            .ok_or_else(|| anyhow!("Missing pczt_signed_tx_hex payload"))?;

        let hex_payload = hex.trim_start_matches("0x").trim();
        let pczt_bytes = hex::decode(hex_payload).map_err(|e| anyhow!("INVALID HEX : {e}"))?;

        Pczt::parse(&pczt_bytes).map_err(|e| anyhow!("Failed to parse PCZT : {e:?}"))
    }
}
