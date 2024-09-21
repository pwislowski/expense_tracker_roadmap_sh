use std::error::Error;

use surrealdb::{engine::remote::ws::Client, Surreal};
use thiserror::Error;

use super::types::{ExpenseRecordDb, ExpenseRecordWithJwt};

#[derive(Error, Debug)]
pub enum DataBaseError {
    #[error("the data for key `{0}` is not found")]
    NoRecordFound(String),
}

const TABLE: &'static str = "expenses";

pub async fn create_expense(
    conn: &Surreal<Client>,
    expense: ExpenseRecordWithJwt,
) -> Result<Vec<ExpenseRecordDb>, Box<dyn Error>> {
    let record: ExpenseRecordDb = expense.into();

    let new = conn.create(TABLE).content(record).await?;

    Ok(new)
}

pub async fn remove_expense(conn: &Surreal<Client>, id: &str) -> Result<(), Box<dyn Error>> {
    let res: Option<ExpenseRecordDb> = conn.delete((TABLE, id)).await?;

    match res {
        Some(_) => Ok(()),
        None => Err(Box::new(DataBaseError::NoRecordFound(id.to_string()))),
    }
}

pub async fn update_expense(conn: &Surreal<Client>, mut update: ExpenseRecordDb) -> Option<()> {
    let record_id = update.id.unwrap().id.to_string();
    let res_del_opt: Result<Option<ExpenseRecordDb>, _> = conn.delete((TABLE, &record_id)).await;

    if res_del_opt.is_err() {
        return None;
    }

    update.id = None;
    let res_new: Option<ExpenseRecordDb> = conn
        .create((TABLE, record_id))
        .content(update)
        .await
        .unwrap();

    match res_new {
        Some(_) => Some(()),
        None => None,
    }
}

pub async fn get_expenses(conn: &Surreal<Client>, username: &str) -> Vec<ExpenseRecordDb> {
    let sql = format!("SELECT * FROM {} WHERE username = $username", TABLE);
    let mut res = conn.query(sql).bind(("username", username)).await.unwrap();
    res.take(0).unwrap()
}
