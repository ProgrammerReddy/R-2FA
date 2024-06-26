pub mod models;
mod schema;

use crate::schema::tokens;
use diesel::{
    pg::PgConnection, prelude::{QueryDsl, RunQueryDsl}, result::Error, Connection, ExpressionMethods, SelectableHelper
};
use dotenv::dotenv;
use models::{NewToken, Token};
use std::env::{self, VarError};

pub fn establish_connection() -> Result<PgConnection, VarError> {
    dotenv().ok();

    let pg_username = env::var("PGUSERNAME")?;
    let password = env::var("PASSWORD")?;
    let host = env::var("HOST")?;
    let pgdb = env::var("PGDB")?;
    let connection = format!("postgres://{}:{}@{}/{}", pg_username, password, host, pgdb);

    Ok(PgConnection::establish(&connection)
        .unwrap_or_else(|_| panic!("Couldn't connect to the database:\n{}", connection))
    )
}

pub fn read_tokens(connection: Result<PgConnection, VarError>) -> Result<Vec<Token>, Error> {
    use self::schema::tokens::dsl::*;
    
    tokens
        .select(Token::as_select())
        .load::<Token>(&mut connection.unwrap())
}

pub fn create_tokens<'a>(
    account_name: &'a str, 
    issuer: &'a str, 
    secret: &'a str, 
    connection: Result<PgConnection, VarError>
) -> Result<Vec<Token>, Error> {
    
    let new_token = NewToken::new(account_name, issuer, secret);

    diesel::insert_into(tokens::table)
        .values(new_token)
        .returning(Token::as_returning())
        .get_results(&mut connection.unwrap())
}

pub fn delete_tokens(id: i32, connection: Result<PgConnection, VarError>) -> Result<usize, Error> {
    diesel::delete(tokens::table)
        .filter(tokens::id.eq(id))
        .execute(&mut connection.unwrap())
}
