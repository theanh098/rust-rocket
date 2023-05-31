use crate::utils;
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::{
  http::Status,
  request::{FromRequest, Outcome, Request},
  serde::{Deserialize, Serialize},
};
use serde::de::DeserializeOwned;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub id: u32,
  pub wallet_address: String,
  pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubClaims {
  pub id: u32,
  pub exp: usize,
}
#[derive(Debug)]
pub struct JwtAuthGuard<T>(pub T);
#[derive(Debug)]
pub struct OptionalClaims(pub Option<Claims>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Tokens {
  access_token: String,
  refresh_token: String,
}

pub fn generate_tokens(id: u32, wallet_address: String, con: &mut redis::Connection) -> Tokens {
  let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
  let refresh_secret = env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET must be set.");

  let expiration_access: i64 = Utc::now()
    .checked_add_signed(chrono::Duration::seconds(60))
    .expect("Invalid timestamp")
    .timestamp();
  let expiration_refresh: i64 = Utc::now()
    .checked_add_signed(chrono::Duration::days(39))
    .expect("Invalid timestamp")
    .timestamp();

  let claims = Claims {
    id,
    wallet_address,
    exp: expiration_access as usize,
  };

  let sub_claims = SubClaims {
    id,
    exp: expiration_refresh as usize,
  };
  let header = Header::new(Algorithm::HS256);

  let secret_key = EncodingKey::from_secret(secret.as_bytes());
  let refresh_key = EncodingKey::from_secret(refresh_secret.as_bytes());

  let access_token = encode(&header, &claims, &secret_key).expect("encode wrong with access_token");
  let refresh_token =
    encode(&header, &sub_claims, &refresh_key).expect("encode wrong with refresh_token");

  redis::cmd("SET")
    .arg(utils::refresh_token_generate(id as usize))
    .arg(&refresh_token)
    .query::<()>(con)
    .expect("set my_key redis fail");

  Tokens {
    access_token,
    refresh_token,
  }
}

pub fn decode_jwt<T: DeserializeOwned>(
  token: &str,
  secret: String,
) -> Result<T, jsonwebtoken::errors::Error> {
  match decode::<T>(
    &token,
    &DecodingKey::from_secret(secret.as_bytes()),
    &Validation::new(Algorithm::HS256),
  ) {
    Ok(decoded) => Ok(decoded.claims),
    Err(err) => Err(err),
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtGuardErr {
  pub message: String,
}

#[rocket::async_trait]
impl<'r, T: DeserializeOwned> FromRequest<'r> for JwtAuthGuard<T> {
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
        let access_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

        match decode_jwt(token, access_secret) {
          Ok(claims) => Outcome::Success(JwtAuthGuard(claims)),

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
#[rocket::async_trait]
impl<'r> FromRequest<'r> for OptionalClaims {
  type Error = ();

  async fn from_request(
    req: &'r Request<'_>,
  ) -> rocket::request::Outcome<OptionalClaims, Self::Error> {
    let bearer_token = req.headers().get_one("authorization");

    match bearer_token {
      None => Outcome::Success(OptionalClaims(None)),

      Some(bearer_token) => {
        let token = bearer_token.trim_start_matches("Bearer").trim();
        let refresh_secret =
          env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET must be set.");

        match decode_jwt(token, refresh_secret) {
          Ok(claims) => Outcome::Success(OptionalClaims(Some(claims))),
          _ => Outcome::Success(OptionalClaims(None)),
        }
      }
    }
  }
}
