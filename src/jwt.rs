use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub id: u32,
  pub wallet_address: String,
  pub exp: usize,
}

pub fn jwt_generator(
  id: u32,
  wallet_address: String,
) -> Result<String, jsonwebtoken::errors::Error> {
  let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

  let expiration: i64 = Utc::now()
    .checked_add_signed(chrono::Duration::seconds(60))
    .expect("Invalid timestamp")
    .timestamp();

  let claims = Claims {
    id,
    wallet_address,
    exp: expiration as usize,
  };

  let header = Header::new(Algorithm::HS256);
  let key = EncodingKey::from_secret(secret.as_bytes());

  encode(&header, &claims, &key)
}

pub fn decode_jwt(token: String) -> Result<Claims, jsonwebtoken::errors::ErrorKind> {
  let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

  match decode::<Claims>(
    &token,
    &DecodingKey::from_secret(secret.as_bytes()),
    &Validation::new(Algorithm::HS512),
  ) {
    Ok(token) => Ok(token.claims),
    Err(err) => Err(err.kind().to_owned()),
  }
}
