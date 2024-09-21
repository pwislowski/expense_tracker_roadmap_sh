use std::error::Error;
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use super::types::{User, UsernameExistsError};
const TABLE: &'static str = "users";

pub async fn get_user(conn: &Surreal<Client>, username: &str) -> Option<User> {
    let sql = format!("SELECT * FROM {} WHERE username = $username", TABLE);

    let mut response = conn.query(sql).bind(("username", username)).await.unwrap();
    let users: Vec<User> = response.take(0).unwrap();

    match users.first() {
        Some(user) => Some(user.clone()),
        None => None,
    }
}

pub async fn create_credentials(
    conn: &Surreal<Client>,
    username: &str,
    passowrd: &str,
) -> Result<Thing, Box<dyn Error>> {
    let username_check = get_user(conn, username).await;
    if username_check.is_some() {
        return Err(UsernameExistsError.into());
    }

    let new_user: Vec<User> = conn
        .create(TABLE)
        .content(User {
            id: None,
            username: username.to_string(),
            password: sha256::digest(passowrd),
        })
        .await?;

    let id = new_user[0].clone().id.unwrap();

    Ok(id)
}
