use rocket::get;
use rocket::serde::json::Json;
use rocket::State;

use crate::prisma::{self, PrismaClient};

prisma::businesses::select!(Bn {
    id
    name
    logo
});

#[get("/businesses")]
pub async fn get_businesses(prisma: &State<PrismaClient>) -> Json<Vec<Bn::Data>> {
  let businesses = prisma
    .businesses()
    .find_many(vec![])
    .take(10)
    .select(Bn::select())
    .exec()
    .await
    .unwrap();

  Json(businesses)
}
