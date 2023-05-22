use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_validation::{Validate, Validated};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct HelloData {
  #[validate(length(min = 1))]
  name: String,
  #[validate(range(min = 0, max = 100))]
  age: u8,
}

#[post("/body", data = "<data>")]
pub fn validated_body(data: Validated<Json<HelloData>>) -> Json<HelloData> {
  Json(data.into_deep_inner())
}
