#![recursion_limit = "256"]

#[macro_use]
extern crate rocket;

mod catchers;
mod jwt;
mod prisma;
mod routes;
mod state;
mod utils;
mod validation;

use routes::{
  auth,
  body::validated_body,
  business::get_businesses,
  query::{query, validated_query},
  review::{create_review, get_review, get_reviews},
};

#[get("/")]
fn hello() -> &'static str {
  "hello my friend"
}

#[launch]
async fn rocket() -> _ {
  rocket::build()
    .manage(state::prisma_client().await)
    .manage(state::redis_client())
    .mount("/", routes![hello])
    .mount("/", routes![get_reviews, create_review, get_review])
    .mount("/", routes![get_businesses])
    .mount("/", routes![validated_body])
    .mount("/", routes![query, validated_query])
    .mount("/", routes![auth::login, auth::renew])
    .register(
      "/",
      catchers![
        validation::validation_catcher,
        catchers::not_found,
        catchers::unauthorized
      ],
    )
}
