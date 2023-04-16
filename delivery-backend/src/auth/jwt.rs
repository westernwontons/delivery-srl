//! JWT authenticaton module
//!
//! The value of the "APP_SECRET" environment variable will be read
//! and used to provide tokens for clients.

use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;

use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::request::Parts;
use axum::{http::StatusCode, response::IntoResponse, Json};
use axum::{RequestPartsExt, TypedHeader};
use chrono::{Duration, Utc};
use jsonwebtoken::errors::{Error as JwtError, ErrorKind};
use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;
use serde_json::json;

use crate::auth::token::verify_and_decode_token;

/// Private and public key pair
pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let private_key_path =
        env::var("PRIVATE_KEY_PATH").expect("Env var PRIVATE_KEY_PATH is not set");

    let public_key_path = env::var("PUBLIC_KEY_PATH").expect("Env var PUBLIC_KEY_PATH is not set");

    let mut private_key = Vec::<u8>::with_capacity(227);
    File::open(private_key_path)
        .expect("Failed to open file containing private key")
        .read_to_end(&mut private_key)
        .expect("Failed to read file containing private key");

    let mut public_key = Vec::<u8>::with_capacity(227);
    File::open(public_key_path)
        .expect("Failed to open file containing public key")
        .read_to_end(&mut public_key)
        .expect("Failed to read file containing public key");

    Keys::from_ec_keys(private_key.as_slice(), public_key.as_slice())
        .expect("Failed to acquire encoding and decoding keys")
});

/// Cryptographic keys to encode and decode a JWT
pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey
}

impl Keys {
    /// Create [`Keys`] from an ECDSA private and public key pair
    ///
    /// It can error if either the private or public key is invalid
    fn from_ec_keys(private: &[u8], public: &[u8]) -> Result<Self, JwtError> {
        let encoding = EncodingKey::from_ec_pem(private)?;
        let decoding = DecodingKey::from_ec_pem(public)?;
        Ok(Self { encoding, decoding })
    }
}

//////////////////////////////////////////////////////////////////////////////////////////

/// The claims in a JWT
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize
}

impl Claims {
    /// Creates a new [`Claims`].
    ///
    /// The `sub` is the unique identifier and exp is the expiration date
    ///
    /// It's `now + 15` minutes by default
    pub fn new(sub: String) -> Self {
        Self { sub, exp: (Utc::now() + Duration::minutes(15)).timestamp() as usize }
    }

    /// Returns a reference to the `sub` of this [`Claims`].
    pub fn sub(&self) -> &str {
        self.sub.as_ref()
    }

    /// Returns the `exp` of this [`Claims`].
    pub fn exp(&self) -> usize {
        self.exp
    }
}

#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for Claims
where
    S: Send + Sync
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let token_data = verify_and_decode_token(bearer.token())?;

        Ok(token_data.claims)
    }
}

//////////////////////////////////////////////////////////////////////////////////////////

/// The response to a successful authentication
#[derive(Debug, serde::Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String
}

#[allow(dead_code)]
impl AuthBody {
    /// Create a new `Bearer: <token>` type [`AuthBody`]
    pub fn new_bearer(access_token: String) -> Self {
        Self { access_token, token_type: "Bearer".into() }
    }

    /// Returns a reference to the `access_token` of this [`AuthBody`].
    pub fn access_token(&self) -> &str {
        self.access_token.as_ref()
    }

    /// Returns a reference to the `token_type` of this [`AuthBody`].
    pub fn token_type(&self) -> &str {
        self.token_type.as_ref()
    }
}

impl IntoResponse for AuthBody {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

//////////////////////////////////////////////////////////////////////////////////////////

/// The possible errors when a user attempts to authenticate
#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken
}

impl std::error::Error for AuthError {}

impl Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            AuthError::WrongCredentials => write!(f, "WrongCredentials"),
            AuthError::MissingCredentials => write!(f, "MissingCredentials"),
            AuthError::TokenCreation => write!(f, "TokenCreation"),
            AuthError::InvalidToken => write!(f, "InvalidToken")
        }
    }
}

impl From<JwtError> for AuthError {
    fn from(value: JwtError) -> Self {
        match value.into_kind() {
            ErrorKind::InvalidToken => Self::InvalidToken,
            ErrorKind::InvalidSignature => Self::InvalidToken,
            ErrorKind::InvalidEcdsaKey => Self::InvalidToken,
            ErrorKind::InvalidRsaKey(_) => Self::InvalidToken,
            ErrorKind::RsaFailedSigning => Self::InvalidToken,
            ErrorKind::InvalidAlgorithmName => Self::InvalidToken,
            ErrorKind::InvalidKeyFormat => Self::InvalidToken,
            ErrorKind::MissingRequiredClaim(_) => Self::MissingCredentials,
            ErrorKind::ExpiredSignature => Self::InvalidToken,
            ErrorKind::InvalidIssuer => Self::InvalidToken,
            ErrorKind::InvalidAudience => Self::InvalidToken,
            ErrorKind::InvalidSubject => Self::InvalidToken,
            ErrorKind::ImmatureSignature => Self::InvalidToken,
            ErrorKind::InvalidAlgorithm => Self::InvalidToken,
            ErrorKind::MissingAlgorithm => Self::InvalidToken,
            ErrorKind::Base64(_) => Self::InvalidToken,
            ErrorKind::Json(_) => Self::InvalidToken,
            ErrorKind::Utf8(_) => Self::InvalidToken,
            ErrorKind::Crypto(_) => Self::InvalidToken,
            _ => Self::InvalidToken
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token")
        };
        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}
