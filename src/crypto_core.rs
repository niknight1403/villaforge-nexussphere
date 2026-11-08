use ring::signature;
use ring::error::Unspecified;

/// Verifies a message signature using Ed25519 to ensure Zero-Knowledge integrity.
/// This is the core of the NexusSphere E2E security layer.
pub fn verify_message_signature(
    public_key_bytes: &[u8],
    message: &[u8],
    signature_bytes: &[u8],
) -> Result<(), Unspecified> {
    let public_key = signature::UnparsedPublicKey::new(
        &signature::ED25519,
        public_key_bytes,
    );

    public_key.verify(message, signature_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ring::signature::KeyPair;

    #[test]
    fn test_signature_verification() {
        let rng = ring::rand::SystemRandom::new();
        let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();
        
        let message = b"nexus_sphere_secure_payload";
        let signature = key_pair.sign(message);
        
        assert!(verify_message_signature(
            key_pair.public_key().as_ref(),
            message,
            signature.as_ref()
        ).is_ok());
    }
}