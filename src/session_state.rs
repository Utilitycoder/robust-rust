use std::future::{ready, Ready};

use actix_session::{Session, SessionExt, SessionGetError, SessionInsertError};
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use uuid::Uuid;

// Purpose: A wrapper around `Session` that provides type-safe access to the
// session data.
pub struct TypedSession(Session);

// Associated functions for `TypedSession`.
impl TypedSession {
    // The key used to store the user ID in the session.
    const USER_ID_KEY: &'static str = "user_id";

    // Renews the session key, assigning existing session state to new key.
    pub fn renew(&self) {
        self.0.renew();
    }

    // Inserts the user ID into the session.
    // Returns an error if it fails to serialize value to JSON.
    pub fn insert_user_id(&self, user_id: Uuid) -> Result<(), SessionInsertError> {
        self.0.insert(Self::USER_ID_KEY, user_id)
    }

    // Gets the user ID from the session.
    // Returns an error if it fails to deserialize value from JSON.
    pub fn get_user_id(&self) -> Result<Option<Uuid>, SessionGetError> {
        self.0.get(Self::USER_ID_KEY)
    }

    // Removes the user ID from the session.
    pub fn log_out(self) {
        self.0.purge()
    }
}

impl FromRequest for TypedSession {
    // This is a complicated way of saying
    // "We return the same error returned by the
    // implementation of `FromRequest` for `Session`".
    type Error = <Session as FromRequest>::Error;
    // Rust does not yet support the `async` syntax in traits.
    // From request expects a `Future` as return type to allow for extractors
    // that need to perform asynchronous operations (e.g. a HTTP call)
    // We do not have a `Future`, because we don't perform any I/O,
    // so we wrap `TypedSession` into `Ready` to convert it into a `Future` that
    // resolves to the wrapped value the first time it's polled by the executor.
    type Future = Ready<Result<TypedSession, Self::Error>>;

    // The `from_request` method is called by actix-web to convert the incoming request into a
    // `TypedSession`.
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(Ok(TypedSession(req.get_session())))
    }
}
