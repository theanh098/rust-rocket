#![recursion_limit = "256"]

#[macro_use]
extern crate rocket;

mod catchers;
mod jwt;
mod prisma;
mod routes;
mod validation;

use prisma::PrismaClient;
use routes::{
  auth,
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
    .mount("/", routes![auth::login])
    .register(
      "/",
      catchers![
        validation::validation_catcher,
        catchers::not_found,
        catchers::unauthorized
      ],
    )
}
