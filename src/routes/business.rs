use redis::Client;
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
pub async fn get_businesses(
  prisma: &State<PrismaClient>,
  redis_client: &State<Client>,
) -> Json<Vec<Bn::Data>> {
  let mut con = redis_client
    .get_connection()
    .expect("getting redis connection fail");

  redis::cmd("SET")
    .arg("my_key")
    .arg(42)
    .query::<()>(&mut con)
    .expect("set my_key redis fail");

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
