use rocket::get;
use rocket::serde::json::Json;
use rocket::State;

use crate::{prisma, Context};

#[get("/")]
pub async fn reviews(ctx: &State<Context>) -> Json<Vec<prisma::reviews::Data>> {
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
