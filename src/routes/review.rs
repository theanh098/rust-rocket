use rocket::serde::json::Json;
use rocket::State;
use rocket::{get, post};

use rocket_validation::{Validate, Validated};
use serde::{Deserialize, Serialize};

use crate::{prisma, Context};

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
  ctx: &State<Context>,
  page: Option<u32>,
  limit: Option<u32>,
  color: Option<Color>,
) -> Json<Vec<prisma::reviews::Data>> {
  dbg!(page);
  dbg!(limit);
  dbg!(color);

  let reviews = ctx
    .prisma
    .reviews()
    .find_many(vec![])
    .take(10)
    .exec()
    .await
    .unwrap();

  Json(reviews)
}

#[post("/reviews", data = "<input>")]
pub async fn create_review(input: Validated<Json<SignupData>>) {
  dbg!(input);
}
