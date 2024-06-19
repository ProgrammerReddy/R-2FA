mod models;
mod schema;

use std::env;
use models::Token;

use diesel::{pg::PgConnection, Connection, SelectableHelper, prelude::{QueryDsl, RunQueryDsl}};
use dotenv::dotenv;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let pg_username = env::var("PGUSERNAME").expect("Couldn't find the env variable.");
    let password = env::var("PASSWORD").expect("Couldn't find the env variable.");
    let host = env::var("HOST").expect("Couldn't find the env variable.");
    let pgdb = env::var("PGDB").expect("Couldn't find the env variable.");

    let connection = format!("postgres://{}:{}@{}/{}", pg_username, password, host, pgdb);
    PgConnection::establish(&connection).unwrap_or_else(|_| panic!("Error connecting to {}", connection))
}

fn read_tokens() -> Vec<Token> {
    use self::schema::tokens::dsl::*;

    let mut connection = establish_connection();
    tokens
        .limit(5)
        .select(Token::as_select())
        .load::<Token>(&mut connection)
        .expect("Error loading tokens")
}

pub fn read_tokens_id() -> Vec<i32> {
    read_tokens().iter().map(|x| x.id).collect::<Vec<i32>>()
}

pub fn read_tokens_account_name() -> String {
    read_tokens().iter().map(|x| x.account_name.clone()).collect::<Vec<String>>().join(", ")
}

pub fn read_tokens_issuer() -> Vec<String> {
    read_tokens().iter().map(|x| x.issuer.clone()).collect::<Vec<String>>()
}

pub fn read_tokens_secret() -> String {
    read_tokens().iter().map(|x| x.secret.clone()).collect::<Vec<String>>().join(", ")
}
