use nexus_sphere_core::crypto::{verify_node_identity, generate_keypair};
use nexus_sphere_core::gateway::{Gateway, Message};

#[test]
fn test_e2e_agent_mesh_communication() {
    // Setup Gateway
    let gateway = Gateway::new();
    let (pk, sk) = generate_keypair();
    
    // Simulate Agent Node
    let node_id = "node_001";
    let msg = Message::new(node_id, "workload_status", "active");
    let signed_msg = msg.sign(&sk);
    
    // Verify E2E Flow
    assert!(verify_node_identity(node_id, &pk, &signed_msg.signature));
    assert!(gateway.process_message(signed_msg).is_ok());
}

#[test]
fn test_circuit_breaker_quarantine() {
    let mut gateway = Gateway::new();
    gateway.report_failure("node_001");
    assert!(gateway.is_quarantined("node_001"));
}