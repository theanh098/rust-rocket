use crate::jwt::jwt_generator;
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
pub struct LoginResponse {
  pub access_token: String,
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
) -> Result<Json<LoginResponse>, status::Custom<Json<LoginError>>> {
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
          let token = jwt_generator(new_user.id as u32, new_user.wallet_address);

          match token {
            Ok(token) => {
              return Ok(Json(LoginResponse {
                access_token: token,
              }));
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

    Some(data) => {
      let token = jwt_generator(data.id as u32, data.wallet_address);
      match token {
        Ok(token_unwrap) => {
          return Ok(Json(LoginResponse {
            access_token: token_unwrap,
          }));
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
  };
}
