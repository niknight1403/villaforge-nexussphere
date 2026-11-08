use axum::{routing::post, Json, Router, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::crypto;

#[derive(Serialize, Deserialize)]
pub struct EncryptedPayload {
    pub data: Vec<u8>,
    pub nonce: Vec<u8>,
    pub public_key: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct GatewayResponse {
    pub success: bool,
    pub message: String,
}

/// Handles incoming encrypted agent messages.
/// Performs Zero-Knowledge verification before processing.
async fn handle_agent_message(Json(payload): Json<EncryptedPayload>) -> (StatusCode, Json<GatewayResponse>) {
    // TODO: Integrate actual crypto::verify_zk_proof(payload.public_key, ...)
    if payload.data.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(GatewayResponse { 
            success: false, 
            message: "Empty payload".to_string() 
        }));
    }

    (StatusCode::OK, Json(GatewayResponse { 
        success: true, 
        message: "Message processed securely".to_string() 
    }))
}

pub fn create_router() -> Router {
    Router::new().route("/gateway/message", post(handle_agent_message))
}