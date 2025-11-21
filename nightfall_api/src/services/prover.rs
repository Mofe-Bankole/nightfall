use crate::models::zcash_models::{PCZTBuilder, PcztJson};
use anyhow::anyhow;
use orchard::builder::{Builder as OrchardBuilder, BundleType};
use std::path::Path;
use tonic::transport::{Channel, Endpoint};
use zcash_proofs::prover::LocalTxProver;
use zcash_protocol::consensus::{Parameters, TEST_NETWORK, TestNetwork};

pub fn build_unsigned(payload: PcztJson) -> Result<(), anyhow::Error> {
    let local_prover = LocalTxProver::new(
        &Path::new("./sapling-spend.params"),
        &Path::new("./sapling-output.params"),
    );

    let anchor = payload
        .prover
        .anchor
        .as_ref()
        .ok_or_else(|| anyhow!("MISSING ORCHARD ANCHOR"))?
        .clone();

    let builder = PCZTBuilder::try_from_json(payload)?;

    // Use the correct types for BundleType::Transactional
    // `flags` expects an orchard::Flags, `bundle_required` expects a bool
    // Here we use orchard::Flags::from_parts(false, false) as an example; replace as needed.
    let bundle_type = BundleType::Transactional {
        flags: orchard::bundle::Flags::from_parts(false, false),
        bundle_required: true,
    };

    let final_build = OrchardBuilder::new(bundle_type, anchor);

    // You might want to actually use builder and final_build here

    Ok(())
}
