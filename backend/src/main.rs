use db_handler::types::{ExpenseId, ExpenseRecordDb, ExpenseRecordWithJwt, Jwt};
use rocket::response::status;
use rocket::serde::json::Json;
use utils::{build_jwt_handler, establish_db_connection};
#[macro_use]
extern crate rocket;

mod db_handler;
mod utils;

#[post("/register", data = "<credentials>")]
async fn register_user(
    mut credentials: Json<db_handler::types::Credentials<'_>>,
) -> Result<status::Accepted<Json<String>>, status::Conflict<String>> {
    let conn = utils::establish_db_connection().await;
    let user_id = db_handler::authentication::create_credentials(
        &conn,
        credentials.username,
        credentials.password,
    )
    .await;

    credentials.password = "";
    let credentials_str = rocket::serde::json::to_string(&credentials.into_inner()).unwrap();

    let jwt_handler = utils::build_jwt_handler();
    let jwt_signed = jwt_handler.sign(&credentials_str).unwrap();

    match user_id {
        Ok(_) => Ok(status::Accepted(Json(jwt_signed))),
        Err(error) => Err(status::Conflict(format!("{}", error))),
    }
}

#[post("/login", data = "<credentials>")]
async fn login_user(
    mut credentials: Json<db_handler::types::Credentials<'_>>,
) -> Result<status::Accepted<Json<String>>, status::Unauthorized<String>> {
    let conn = utils::establish_db_connection().await;
    let user_id = db_handler::authentication::get_user(&conn, credentials.username).await;

    if user_id.is_none() {
        return Err(status::Unauthorized(format!(
            "username: '{}' not found",
            credentials.username
        )));
    }

    let password_given_hash = sha256::digest(credentials.password);

    credentials.password = "";
    let credentials_str = rocket::serde::json::to_string(&credentials.into_inner()).unwrap();

    let jwt_handler = utils::build_jwt_handler();
    let jwt_signed = jwt_handler.sign(&credentials_str).unwrap();

    let password_stored_hash = user_id.unwrap().password;

    match password_stored_hash == password_given_hash {
        true => Ok(status::Accepted(Json(jwt_signed))),
        false => Err(status::Unauthorized(format!("Password incorrect"))),
    }
}

#[post("/verify", data = "<jwt>")]
async fn verify_jwt(
    jwt: Json<db_handler::types::Jwt<'_>>,
) -> Result<status::Accepted<String>, status::Unauthorized<String>> {
    let conn = utils::establish_db_connection().await;
    let jwt_handler = utils::build_jwt_handler();
    let token_str = jwt.auth;
    let user = jwt_handler.decode(token_str).unwrap();
    let db_user_option = db_handler::authentication::get_user(&conn, &user.username).await;

    if db_user_option.is_none() {
        Err(status::Unauthorized(format!("User does not exist.")))
    } else {
        Ok(status::Accepted(String::from("Authenticated")))
    }
}

#[post("/expense/new", data = "<expense>")]
async fn new_expense(
    expense: Json<ExpenseRecordWithJwt>,
) -> Result<Json<Vec<ExpenseRecordDb>>, Json<String>> {
    let conn = utils::establish_db_connection().await;

    let res = db_handler::expenses::create_expense(&conn, expense.into_inner()).await;

    match res {
        Ok(v) => Ok(Json(v)),
        Err(_) => Err(Json("Failure".into())),
    }
}

#[post("/expense/get", data = "<token>")]
async fn get_expenses(token: Json<Jwt<'_>>) -> Json<Vec<ExpenseRecordDb>> {
    let jwt_handler = build_jwt_handler();
    let conn = establish_db_connection().await;
    let user = jwt_handler.decode(token.auth).unwrap();
    let res = db_handler::expenses::get_expenses(&conn, &user.username).await;

    Json(res)
}

#[delete("/expense/remove", data = "<id>")]
async fn remove_expense(
    id: Json<ExpenseId<'_>>,
) -> Result<status::Accepted<()>, status::BadRequest<()>> {
    let conn = establish_db_connection().await;
    let id_str = id.id;

    match db_handler::expenses::remove_expense(&conn, id_str).await {
        Ok(_) => Ok(status::Accepted(())),
        Err(_) => Err(status::BadRequest(())),
    }
}

#[get("/")]
fn index() -> &'static str {
    "root"
}

#[put("/expense/update", data = "<expense>")]
async fn update_expense(
    expense: Json<ExpenseRecordDb>,
) -> Result<status::Accepted<()>, status::BadRequest<()>> {
    let conn = establish_db_connection().await;
    let res = db_handler::expenses::update_expense(&conn, expense.into_inner()).await;

    match res {
        Some(_) => Ok(status::Accepted(())),
        None => Err(status::BadRequest(())),
    }
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            index,
            register_user,
            login_user,
            verify_jwt,
            new_expense,
            get_expenses,
            remove_expense,
            update_expense
        ],
    )
}
