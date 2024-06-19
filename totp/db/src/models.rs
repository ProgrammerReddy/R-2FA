use diesel::{deserialize::Queryable, pg, Selectable};

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::tokens)]
#[diesel(check_for_backend(pg::Pg))]
pub struct Token {
    pub id: i32,
    pub account_name: String,
    pub issuer: String,
    pub secret: String,
}
