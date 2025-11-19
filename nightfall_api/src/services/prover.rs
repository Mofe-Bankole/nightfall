// use crate::models::zcash_models::{PCZTBuilder, PcztJson};
// use crate::services::pzct_service;
// use axum::Json;
// use orchard::builder::{Builder, BundleType};
// use pczt::Pczt;
// use std::path::Path;
// use zcash_proofs::prover::{self, LocalTxProver};

// pub fn build_unsigned(Json(payload): Json<PcztJson>) -> Result<(), anyhow::Error> {
//     let local_prover = LocalTxProver::new(
//         &Path::new("./sapling-spend.params"),
//         &Path::new("./sapling-output.params"),
//     );
//     let builder = PCZTBuilder::try_from_json(payload);
//     let final_build = orchard::builder::Builder::new(BundleType::Transactional { flags: (), bundle_required: () }), anchor)
//     let ver_keys = local_prover.verifying_keys();

//     Ok(())
// }
