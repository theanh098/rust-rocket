use crate::jwt::jwt_generator;
use crate::prisma;
use crate::{validation::Validated, Context};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
struct LoginRequest {
  wallet_address: String,
}

#[post("/login", data = "<user>")]
pub async fn login(ctx: &State<Context>, user: Json<LoginRequest>) -> Result<String, ()> {
  let user = ctx
    .prisma
    .users()
    .find_unique(prisma::users::wallet_address::equals(
      user.wallet_address.to_string(),
    ))
    .exec()
    .await
    .unwrap();

  let token = match user {
    Some(data) => jwt_generator(data.id as u32, data.wallet_address),
    None => Err(()),
  };

  Ok(String::from("asg"))
}
