use crate::jwt::generate_tokens;
use crate::jwt::JwtAuthGuard;
use crate::jwt::SubClaims;
use crate::jwt::Tokens;
use crate::prisma;
use crate::prisma::PrismaClient;
use crate::utils;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LoginRequest {
  pub wallet_address: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct LoginError {
  pub message: String,
  pub status: u32,
}

#[post("/auth/login", data = "<body>")]
pub async fn login(
  prisma: &State<PrismaClient>,
  body: Json<LoginRequest>,
  redis: &State<redis::Client>,
) -> Result<Json<Tokens>, status::Custom<Json<LoginError>>> {
  let user = prisma
    .users()
    .find_unique(prisma::users::wallet_address::equals(
      body.wallet_address.to_string(),
    ))
    .exec()
    .await
    .unwrap_or(None);

  let mut con = redis
    .get_connection()
    .expect("getting redis connection fail");

  match user {
    None => {
      let new_user = prisma
        .users()
        .create(body.wallet_address.to_string(), vec![])
        .exec()
        .await;

      match new_user {
        Ok(new_user) => {
          let tokens = generate_tokens(new_user.id as u32, new_user.wallet_address);

          redis::cmd("SET")
            .arg(utils::refresh_token_generate(new_user.id as usize))
            .arg(&tokens.refresh_token)
            .query::<()>(&mut con)
            .expect("set my_key redis fail");

          return Ok(Json(tokens));
        }
        Err(err) => {
          return Err(status::Custom(
            Status::InternalServerError,
            Json(LoginError {
              message: err.to_string(),
              status: 500,
            }),
          ));
        }
      }
    }

    Some(user) => {
      let tokens = generate_tokens(user.id as u32, user.wallet_address);

      redis::cmd("SET")
        .arg(utils::refresh_token_generate(user.id as usize))
        .arg(&tokens.refresh_token)
        .query::<()>(&mut con)
        .expect("set my_key redis fail");

      return Ok(Json(tokens));
    }
  };
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RenewRequest {
  pub refresh_token: String,
}

#[post("/auth/renew", data = "<body>")]
pub async fn renew(
  body: Json<RenewRequest>,
  redis: &State<redis::Client>,
  sub_claims: JwtAuthGuard<SubClaims>,
) {
  let mut con = redis
    .get_connection()
    .expect("getting redis connection fail");

  let refresh_token_store = redis::cmd("GET")
    .arg(utils::refresh_token_generate(sub_claims.0.id))
    .query::<String>(&mut con);
}
