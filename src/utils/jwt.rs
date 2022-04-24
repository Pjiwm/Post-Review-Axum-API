use crate::models;
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Display;
/// Gives back a jsonwebtoken by building a new claim with the given user.
pub fn encode_user(user: models::User) -> String {
    let claims = Claims {
        user,
        exp: 200000000000000000,
    };
    let str = encode(&Header::default(), &claims, &KEYS.encoding).unwrap();
    str
}
/// Decodes a jsonwebtoken back to claims, if this goes wrong it will become an AuthError.
pub fn decode_jwt(token: &str) -> Result<Claims, AuthError> {
    let token_data = decode::<Claims>(token, &KEYS.decoding, &Validation::default())
        .map_err(|_| AuthError::InvalidToken)?;

    Ok(token_data.claims)
}
/// Creates our Keys, it grabs the env var JWT_SECRET which is used to encrypt the values
/// When the env JWT_SECRET is not set it will just panic.
static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});
/// Struct for keys
struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    /// Constructor for keys
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
/// Claims are used for jsonwebtokens
/// Claims consist out of a user and an expire time (exp)
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user: models::User,
    exp: usize,
}
impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "user: {:?}\n", self.user)
    }
}

#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;
        Ok(token_data.claims)
    }
}
/// Enum for different types of auth errors
#[derive(Debug)]
#[allow(dead_code)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}
/// If a jsonwebtoken is not present but the method uses claims this will immediately give back an AuthError.
impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
