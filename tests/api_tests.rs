use reqwest::Client;
use tokio;

#[tokio::test]
async fn test_get_existing_order() {
    let client = Client::new();
    let base_url = "http://localhost:3000/orders";
    let order_id = "b563feb7b2b84b6test";

    let response = client
        .get(&format!("{}/{}", base_url, order_id))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200, "Expected status 200 for existing order ID");

    let body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response body");

    assert!(body["order_uid"] == order_id, "Expected correct order ID in response");
}

#[tokio::test]
async fn test_get_nonexistent_order() {
    let client = Client::new();
    let base_url = "http://localhost:3000/orders";
    let nonexistent_order_id = "nonexistentId";

    let response = client
        .get(&format!("{}/{}", base_url, nonexistent_order_id))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 404, "Expected status 404 for non-existent order ID");
}

#[tokio::test]
async fn test_get_invalid_order_id() {
    let client = Client::new();
    let base_url = "http://localhost:3000/orders";
    let invalid_format_order_id = "!@#$%&*<>";

    let response = client
        .get(&format!("{}/{}", base_url, invalid_format_order_id))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 400, "Expected status 400 for invalid order ID format");
}
