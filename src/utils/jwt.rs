use crate::models;
use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};
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
pub fn decode_jwt(token: &str) -> Result<Claims, Response> {
    let token_data = decode::<Claims>(token, &KEYS.decoding, &Validation::default());
    if let Ok(token) = token_data {
        Ok(token.claims)
    } else {
        Err((StatusCode::UNAUTHORIZED, Json(json!("invalid token"))).into_response())
    }
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
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<Value>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| {
                    (
                        StatusCode::UNAUTHORIZED,
                        Json(json!({"error": "unauthorized"})),
                    )
                })?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "unauthorized"})),
            )
        })?;
        Ok(token_data.claims)
    }
}
