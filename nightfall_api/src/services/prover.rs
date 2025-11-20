use crate::models::zcash_models::{PCZTBuilder, PcztJson};
use crate::services::pzct_service;
use anyhow::anyhow;
use axum::Json;
use orchard::builder::{Builder as OrchardBuilder, BundleType};
use pczt::Pczt;
use std::ops::Deref;
use std::path::Path;
use tonic::transport::{Channel, Endpoint};
use zcash_proofs::prover::LocalTxProver;
use zcash_protocol::consensus::{Parameters, TEST_NETWORK, TestNetwork};
use zcash_protocol::value::Zatoshis;
use zcash_transparent::address::TransparentAddress;

pub fn build_unsigned(Json(payload): Json<PcztJson>) -> Result<(), anyhow::Error> {
    let local_prover = LocalTxProver::new(
        &Path::new("./sapling-spend.params"),
        &Path::new("./sapling-output.params"),
    );

    let ver_key = local_prover.verifying_keys();
    let anchor = payload
        .prover
        .anchor
        .as_ref()
        .ok_or_else(|| anyhow!("MISSING ORCHARD ANCHOR"))?;
    let builder = PCZTBuilder::try_from_json(payload);
    let bundle_type = BundleType::Transactional {
        flags: (),
        bundle_required: (),
    };

    let final_build = OrchardBuilder::new(bundle_type, anchor.clone());
    let ver_keys = local_prover.verifying_keys();

    Ok(())
}
