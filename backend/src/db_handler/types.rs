use core::fmt;
use std::error::Error;

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize)]
pub struct Credentials<'r> {
    pub username: &'r str,
    pub password: &'r str,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Jwt<'r> {
    pub auth: &'r str,
}

#[derive(Clone, Eq, PartialEq)]
pub struct UsernameExistsError;

impl Error for UsernameExistsError {}
impl fmt::Display for UsernameExistsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Username already in use")
    }
}

impl fmt::Debug for UsernameExistsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

pub struct PasswordMismatchError;

impl Error for PasswordMismatchError {}

impl fmt::Display for PasswordMismatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Incorrect password")
    }
}

impl fmt::Debug for PasswordMismatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(skip_serializing)]
    pub id: Option<Thing>,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpenseRecordWithJwt {
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    pub id: Option<Thing>,
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    pub username: Option<String>,
    pub name: String,
    pub value: String,
    pub category: String,
    pub date: ExpenseDate,
    pub jwt_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpenseDate {
    pub calendar: Calendar,
    pub era: String,
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Calendar {
    pub identifier: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpenseRecordDb {
    pub id: Option<Thing>,
    pub username: String,
    pub name: String,
    pub value: String,
    pub category: String,
    pub date: ExpenseDate,
}

impl From<ExpenseRecordWithJwt> for ExpenseRecordDb {
    fn from(value: ExpenseRecordWithJwt) -> Self {
        let jwt_handler = crate::utils::build_jwt_handler();
        let user = jwt_handler.decode(&value.jwt_token).unwrap();

        Self {
            id: None,
            username: user.username,
            name: value.name,
            value: value.value,
            category: value.category,
            date: value.date,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExpenseId<'r> {
    pub id: &'r str,
}
