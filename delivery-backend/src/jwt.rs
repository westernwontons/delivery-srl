//! JWT authenticaton module
//!
//! The value of the "APP_SECRET" environment variable will be read
//! and used to provide tokens for clients.

use std::env;
use std::fs::File;
use std::io::Read;

use axum::{http::StatusCode, response::IntoResponse, Json};
use chrono::{Duration, Utc};
use jsonwebtoken::errors::{Error as JwtError, ErrorKind};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation
};
use once_cell::sync::Lazy;
use serde_json::json;

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

/// Generate a JWT where `sub` is the username
pub fn generate_token(username: &str) -> String {
    let claims = Claims::new(username.to_owned());
    encode(&Header::new(Algorithm::ES256), &claims, &KEYS.encoding)
        .expect("Failed to generate token")
}

/// Decode and verify a JWT token
///
/// Returns the entire decoded token (header and claims)
pub fn verify_and_decode_token(token: &str) -> Result<TokenData<Claims>, JwtError> {
    Ok(decode::<Claims>(token, &KEYS.decoding, &Validation::new(Algorithm::ES256))?)
}

/// Keys to encode and decode a JWT
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

/// The claims in a JWT
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize
}

impl Claims {
    /// Creates a new [`Claims`].
    ///
    /// The `sub` is the unique identifier
    fn new(sub: String) -> Self {
        Self { sub, exp: (Utc::now() + Duration::minutes(15)).timestamp() as usize }
    }
}

/// The response to a successful authentication
///
/// Note: The type of the token is always `Bearer`
#[derive(Debug, serde::Serialize)]
struct AuthBody {
    pub access_token: String,
    pub token_type: String
}

#[allow(dead_code)]
impl AuthBody {
    fn new(access_token: String) -> Self {
        Self { access_token, token_type: "Bearer".into() }
    }
}

/// The possible errors when a user attempts to authenticate
#[allow(dead_code)]
#[derive(Debug)]
enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken
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
