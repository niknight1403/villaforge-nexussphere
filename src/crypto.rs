use ring::signature;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SignedPayload {
    pub data: Vec<u8>,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}

pub fn verify_signature(payload: &SignedPayload) -> bool {
    let peer_public_key = signature::UnparsedPublicKey::new(
        &signature::ED25519,
        &payload.public_key,
    );

    peer_public_key
        .verify(&payload.data, &payload.signature)
        .is_ok()
}