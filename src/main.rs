#[macro_use]
extern crate rocket;

mod prisma;
mod routes;

use prisma::PrismaClient;
use routes::{business, review};

pub struct Context {
    pub prisma: PrismaClient,
}

#[launch]
async fn rocket() -> _ {
    let prisma = PrismaClient::_builder().build().await.unwrap();
    rocket::build()
        .manage(Context { prisma })
        .mount("/reviews", routes![review::reviews])
        .mount("/businesses", routes![business::businesses])
}
