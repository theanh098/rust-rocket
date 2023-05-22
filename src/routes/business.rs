use rocket::get;
use rocket::serde::json::Json;
use rocket::State;

use crate::{prisma, Context};

prisma::businesses::select!(Bn {
    id
    name
    logo
});

#[get("/")]
pub async fn businesses(ctx: &State<Context>) -> Json<Vec<Bn::Data>> {
    let businesses = ctx
        .prisma
        .businesses()
        .find_many(vec![])
        .take(10)
        .select(Bn::select())
        .exec()
        .await
        .unwrap();

    Json(businesses)
}
