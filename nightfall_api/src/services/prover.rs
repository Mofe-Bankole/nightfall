use axum::Json;

use crate::models::zcash_models::PcztJson;

pub fn generate_proof(Json(payload): Json<PcztJson>) {}
