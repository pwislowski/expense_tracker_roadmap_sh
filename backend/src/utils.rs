use std::env;
use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub async fn establish_db_connection() -> Surreal<Client> {
    let db = Surreal::new::<Ws>("surrealdb:8000").await.unwrap();

    db.signin(Root {
        username: &env::var("DB_USER").unwrap(),
        password: &env::var("DB_PASSWORD").unwrap(),
    })
    .await
    .unwrap();

    db.use_ns("test").use_db("test").await.unwrap();

    db
}

pub fn build_jwt_handler() -> super::db_handler::jwt::JwtHandler {
    let secret = "hello_world";

    super::db_handler::jwt::JwtHandler::new(secret.to_string())
}
