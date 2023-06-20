use crate::helpers::spawn_app;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_for_valid_data() {
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.mock_server)
        .await;

    let response = app.post_subscriptions(body.into()).await;
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_details = spawn_app().await;
    let connection = &app_details.db_pool;

    // Act
    let body = "name=le%20guid&email=ursula_le_guin%40gmail.com";
    let response = app_details.post_subscriptions(body.into()).await;
    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(connection)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guid");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app_details = spawn_app().await;
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = app_details.post_subscriptions(invalid_body.into()).await;
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_empty() {
    // Arrange
    let app_details = spawn_app().await;
    let test_cases = vec![
        ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
        ("name=le%20guin&email=", "empty email"),
        (
            "name=le%20guin&email=definitely_not_an_email",
            "invalid email",
        ),
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = app_details.post_subscriptions(invalid_body.into()).await;
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not return a 400 bad request when the payload was {}.",
            error_message
        );
    }
}
