#![recursion_limit = "256"]

#[macro_use]
extern crate rocket;

mod prisma;
mod routes;

use prisma::PrismaClient;
use rocket_validation::CachedValidationErrors;
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

use rocket::Request;

#[catch(404)]
fn not_found(req: &Request) {
  dbg!(req.local_cache(|| CachedValidationErrors(None)).0.as_ref());
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
    .register(
      "/",
      catchers![rocket_validation::validation_catcher, not_found],
    )
}
