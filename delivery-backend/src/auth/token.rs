use super::jwt::{Claims, KEYS};
use jsonwebtoken::errors::Error as JwtError;
use jsonwebtoken::{decode, encode, Algorithm, Header, TokenData, Validation};

/// Generate a JWT
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
