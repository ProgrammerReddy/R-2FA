use crate::{Otp, Token};
use db::{establish_connection, read_tokens};

pub fn env_as_token() -> Token {
    let connection = establish_connection();
    let select_tokens = read_tokens(connection).unwrap_or_default();

    let issuer = select_tokens
        .iter()
        .map(|x| x.issuer.to_string())
        .collect();

    let account_name = select_tokens
        .iter()
        .map(|x| x.account_name.to_string())
        .collect();

    let secret = select_tokens
        .iter()
        .map(|x| x.secret.to_string())
        .collect();

    Token::new(issuer, account_name, secret)
}

pub fn auth(token: Token, otp: Otp) -> String {
    format!(
        "otpauth://totp/{}:{}@?secret={}&issuer={}&algorithm={}&digits={}&period={}",
        token.issuer,
        token.account_name,
        token.secret,
        token.issuer,
        otp.algorithm,
        otp.digits,
        otp.step
    )
}
