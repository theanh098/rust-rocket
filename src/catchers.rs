use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::Request;

use crate::jwt::JwtGuardErr;

#[derive(Serialize, Deserialize)]
pub struct ExceptionResponse {
  code: u16,
  message: String,
}

#[catch(404)]
pub fn not_found() -> Json<ExceptionResponse> {
  Json(ExceptionResponse {
    code: 404,
    message: String::from("Bad request or not found"),
  })
}

#[catch(401)]
pub fn unauthorized(req: &Request) -> Json<ExceptionResponse> {
  Json(ExceptionResponse {
    code: 401,
    message: req
      .local_cache(|| JwtGuardErr {
        message: String::from("Unauthorized"),
      })
      .message
      .to_owned(),
  })
}
