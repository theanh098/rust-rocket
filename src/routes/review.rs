use crate::jwt::{Claims, JwtAuthGuard, OptionalClaims};
use crate::prisma::{reviews, PrismaClient};
use crate::validation::Validated;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{get, post};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, PartialEq, FromFormField)]
pub enum Color {
  Blue,
  Green,
  Red,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct SignupData {
  #[validate(email)]
  mail: String,

  #[validate(url)]
  site: String,

  #[validate(range(min = 18, max = 20))]
  age: u32,
}

#[get("/reviews?<page>&<limit>&<color>")]
pub async fn get_reviews(
  prisma: &State<PrismaClient>,
  page: Option<u32>,
  limit: Option<u32>,
  color: Option<Color>,
) -> Json<Vec<reviews::Data>> {
  dbg!(page);
  dbg!(limit);
  dbg!(color);

  let reviews = prisma
    .reviews()
    .find_many(vec![])
    .take(10)
    .exec()
    .await
    .unwrap();

  Json(reviews)
}

#[post("/reviews", data = "<input>")]
pub async fn create_review(input: Validated<Json<SignupData>>, claims: JwtAuthGuard<Claims>) {
  dbg!(input);
  dbg!(claims);
}

#[get("/reviews/<id>")]
pub async fn get_review(id: usize, claims: OptionalClaims) {
  dbg!(claims);
  dbg!(id);
}
