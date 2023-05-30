use crate::prisma::PrismaClient;
use redis::Client;

/**
 * State {
 *    PrismaClient
 *    RedisPooledConnection
 * }
 */

pub async fn prisma_client() -> PrismaClient {
  PrismaClient::_builder()
    .build()
    .await
    .expect("creating prisma was wrong")
}

pub fn redis_client() -> Client {
  redis::Client::open("redis://127.0.0.1/").expect("opening redis client fail")
}
