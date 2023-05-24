use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::{
  http::Status,
  request::{FromRequest, Outcome, Request},
  serde::{Deserialize, Serialize},
};
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

pub fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
  let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

  match decode::<Claims>(
    &token,
    &DecodingKey::from_secret(secret.as_bytes()),
    &Validation::new(Algorithm::HS256),
  ) {
    Ok(token) => Ok(token.claims),
    Err(err) => Err(err),
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtGuardErr {
  pub message: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
  type Error = ();
  async fn from_request(req: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
    let bearer_token = req.headers().get_one("authorization");
    match bearer_token {
      None => {
        req.local_cache(|| JwtGuardErr {
          message: String::from("Token not found"),
        });

        Outcome::Failure((Status::Unauthorized, ()))
      }
      Some(bearer_token) => {
        let token = bearer_token.trim_start_matches("Bearer").trim();
        match decode_jwt(token) {
          Ok(claims) => Outcome::Success(claims),
          Err(err) => {
            req.local_cache(|| JwtGuardErr {
              message: err.to_string(),
            });
            Outcome::Failure((Status::Unauthorized, ()))
          }
        }
      }
    }
  }
}
