use nexus_sphere_core::crypto::{generate_keypair, sign_message, verify_signature};

#[test]
fn test_signature_lifecycle() {
    let (public_key, secret_key) = generate_keypair();
    let message = b"NexusSphere_Secure_Payload";
    let signature = sign_message(&secret_key, message);
    
    assert!(verify_signature(&public_key, message, &signature), "Signature verification failed");
}

#[test]
fn test_invalid_signature() {
    let (public_key, _) = generate_keypair();
    let message = b"NexusSphere_Secure_Payload";
    let fake_signature = vec![0u8; 64];
    
    assert!(!verify_signature(&public_key, message, &fake_signature), "Invalid signature was accepted");
}