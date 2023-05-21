#[macro_use]
extern crate rocket;

mod prisma;

use prisma::PrismaClient;
use prisma_client_rust::NewClientError;
use rocket::serde::json::Json;

#[get("/")]
async fn index() -> Json<Vec<prisma::reviews::Data>> {
    let client: Result<PrismaClient, NewClientError> = PrismaClient::_builder().build().await;

    let reviews = client
        .unwrap()
        .reviews()
        .find_many(vec![])
        .take(10)
        .exec()
        .await
        .unwrap();

    Json(reviews)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
