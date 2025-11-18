use crate::models::zcash_models::*;
use anyhow::{Result, anyhow};
use pczt::Pczt;
use zcash_client_backend::encoding::decode_payment_address;
use zcash_primitives::transaction::components::transparent::pczt;

/// Module responsible for converting json passed pczt to PROD-READY PCZT
impl TryFrom<PcztJson> for Pczt {
    type Error = anyhow::Error;

    fn try_from(data: PcztJson) -> Result<Self> {
        // Validate and decode all shielded outputs
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

        // Extract the hex string
        let pczt_hex = data
            .prover
            .pczt_signed_tx_hex
            .ok_or_else(|| anyhow!("Missing pczt_signed_tx_hex payload"))?;

        // strip 0x
        let hex_payload = pczt_hex.trim_start_matches("0x").trim();

        let pczt_bytes = hex::decode(hex_payload)
            .map_err(|e| anyhow!("pczt_signed_tx_hex is not valid hex: {e}"))?;

        Pczt::parse(&pczt_bytes).map_err(|e| anyhow!("Failed to parse PCZT container: {e:?}"))
    }
}
