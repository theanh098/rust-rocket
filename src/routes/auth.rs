use crate::jwt::generate_tokens;
use crate::jwt::Tokens;
use crate::prisma;
use crate::Context;
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
  ctx: &State<Context>,
  body: Json<LoginRequest>,
) -> Result<Json<Tokens>, status::Custom<Json<LoginError>>> {
  let user = ctx
    .prisma
    .users()
    .find_unique(prisma::users::wallet_address::equals(
      body.wallet_address.to_string(),
    ))
    .exec()
    .await
    .unwrap_or(None);

  match user {
    None => {
      let new_user = ctx
        .prisma
        .users()
        .create(body.wallet_address.to_string(), vec![])
        .exec()
        .await;

      match new_user {
        Ok(new_user) => {
          let tokens = generate_tokens(new_user.id as u32, new_user.wallet_address);
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
      return Ok(Json(tokens));
    }
  };
}
