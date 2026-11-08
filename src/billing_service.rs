use serde::{Deserialize, Serialize};
use stripe::{Client, CheckoutSession, CheckoutSessionParams, Currency};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionRequest {
    pub user_id: String,
    pub price_id: String,
}

pub struct BillingManager {
    client: Client,
}

impl BillingManager {
    pub fn new(api_key: String) -> Self {
        Self { client: Client::new(api_key) }
    }

    pub async fn create_checkout_session(&self, req: SubscriptionRequest) -> Result<String, String> {
        let params = CheckoutSessionParams {
            success_url: Some("https://nexussphere.io/success"),
            cancel_url: Some("https://nexussphere.io/cancel"),
            mode: Some(stripe::CheckoutSessionMode::Subscription),
            line_items: Some(vec![stripe::CheckoutSessionLineItemParams {
                price: Some(req.price_id),
                quantity: Some(1),
                ..Default::default()
            }]),
            ..Default::default()
        };

        match CheckoutSession::create(&self.client, params).await {
            Ok(session) => Ok(session.url.unwrap_or_default()),
            Err(e) => Err(format!("Stripe API Error: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_billing_initialization() {
        let manager = BillingManager::new("sk_test_mock".to_string());
        assert!(!manager.client.api_key().is_empty());
    }
}