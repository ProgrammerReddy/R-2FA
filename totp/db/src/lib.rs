pub mod models;
mod schema;

use crate::schema::tokens;
use diesel::{
    pg::PgConnection, prelude::{QueryDsl, RunQueryDsl}, Connection, ExpressionMethods, SelectableHelper
};
use dotenv::dotenv;
use models::{NewToken, Token};
use std::env;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let pg_username = env::var("PGUSERNAME").expect("Couldn't find the PGUSERNAME variable.");
    let password = env::var("PASSWORD").expect("Couldn't find the PASSWORD variable.");
    let host = env::var("HOST").expect("Couldn't find the HOST variable.");
    let pgdb = env::var("PGDB").expect("Couldn't find the PGDB variable.");
    let connection = format!("postgres://{}:{}@{}/{}", pg_username, password, host, pgdb);

    PgConnection::establish(&connection)
        .unwrap_or_else(|_| panic!("Error connecting to {}", connection))
}

pub fn read_tokens() -> Vec<Token> {
    use self::schema::tokens::dsl::*;

    let mut connection = establish_connection();
    
    tokens
        .select(Token::as_select())
        .load::<Token>(&mut connection)
        .expect("Error reading tokens")
}

pub fn create_tokens<'a>(account_name: &'a str, issuer: &'a str, secret: &'a str) -> Vec<Token> {
    let mut connection = establish_connection();
    let new_token = NewToken::new(account_name, issuer, secret);

    diesel::insert_into(tokens::table)
        .values(new_token)
        .returning(Token::as_returning())
        .get_results(&mut connection)
        .expect("Error creating token")
}

pub fn delete_tokens(id: i32) -> usize {
    let mut connection = establish_connection();

    diesel::delete(tokens::table)
        .filter(tokens::id.eq(id))
        .execute(&mut connection)
        .expect("Error deleting token")
}
