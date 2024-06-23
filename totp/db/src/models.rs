use crate::schema::tokens;
use diesel::{deserialize::Queryable, pg, prelude::Insertable, Selectable};
use serde::{Serialize, Deserialize};

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = tokens)]
#[diesel(check_for_backend(pg::Pg))]
pub struct Token {
    pub id: i32,
    pub account_name: String,
    pub issuer: String,
    pub secret: String,
}

#[derive(Debug, Insertable, Queryable)]
#[diesel(table_name = tokens)]
pub struct NewToken<'a> {
    pub account_name: &'a str,
    pub issuer: &'a str,
    pub secret: &'a str,
}

impl<'a> NewToken<'a> {
    pub fn new(account_name: &'a str, issuer: &'a str, secret: &'a str) -> Self {
        Self {
            account_name,
            issuer,
            secret,
        }
    }
}
