use crate::validation::Validated;
use rocket::serde::{json::Json, Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, PartialEq, FromFormField, Serialize, Deserialize)]
pub enum Color {
  Red,
  Blue,
  Green,
}

#[derive(Debug, Deserialize, Serialize, Validate, FromForm)]
pub struct HelloData {
  #[validate(length(min = 3))]
  name: String,

  #[validate(range(min = 0, max = 100))]
  age: Option<u8>,

  color: Color,
}

#[get("/query?<name>&<age>&<color>")]
pub fn query(name: String, age: Option<u8>, color: Color) -> Json<HelloData> {
  Json(HelloData { name, age, color })
}

#[get("/validated-query?<params..>")]
pub fn validated_query(params: Validated<HelloData>) -> Json<HelloData> {
  Json(params.into_inner())
}
