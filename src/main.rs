#![recursion_limit = "256"]

#[macro_use]
extern crate rocket;

mod jwt;
mod prisma;
mod routes;
mod validation;

use prisma::PrismaClient;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use routes::{
  body::validated_body,
  business::get_businesses,
  query::{query, validated_query},
  review::{create_review, get_reviews},
};

pub struct Context {
  pub prisma: PrismaClient,
}
#[get("/")]
fn hello() -> &'static str {
  "hello my friend"
}
#[derive(Serialize, Deserialize)]
struct NotFoundErr {
  code: u16,
  message: String,
}

#[catch(404)]
fn not_found() -> Json<NotFoundErr> {
  Json(NotFoundErr {
    code: 404,
    message: String::from("Bad request or not found"),
  })
}

#[launch]
async fn rocket() -> _ {
  let prisma = PrismaClient::_builder().build().await.unwrap();
  rocket::build()
    .manage(Context { prisma })
    .mount("/", routes![hello])
    .mount("/", routes![get_reviews, create_review])
    .mount("/", routes![get_businesses])
    .mount("/", routes![validated_body])
    .mount("/", routes![query, validated_query])
    .register("/", catchers![validation::validation_catcher, not_found])
}
