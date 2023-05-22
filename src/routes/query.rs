use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_validation::{Validate, Validated};

#[derive(Debug, Deserialize, Serialize, Validate, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct HelloData {
  #[validate(length(min = 3))]
  name: String,
  #[validate(range(min = 0, max = 100))]
  age: u8,
}

#[get("/query?<name>&<age>")]
pub fn query(name: String, age: u8) -> Json<HelloData> {
  Json(HelloData { name, age })
}

#[get("/validated-query?<params..>")]
pub fn validated_query(params: Validated<HelloData>) -> Json<HelloData> {
  Json(params.into_inner())
}
