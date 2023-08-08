use reqwest::Client;

use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app_details = spawn_app().await;
    // We need to bring in `reqwest`
    // to perform HTTP requests against our
    let client = Client::new();
    // Act
    let response = client
        .get(&format!("{}/health_check", &app_details.address))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
