use reqwest::Client;
use robust_rust::run;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    // We need to bring in `reqwest`
    // to perform HTTP requests against our
    let client = Client::new();
    // Act
    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(19), response.content_length());
}


#[allow(clippy::let_underscore_future)]
fn spawn_app() {
    let server = run().expect("Failed to bind address");

    let _ = tokio::spawn(server);
}
