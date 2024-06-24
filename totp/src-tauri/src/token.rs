use crate::{Otp, Token};
use db::read_tokens;

pub fn env_as_token() -> Token {
    let issuer = read_tokens()
        .unwrap_or_default()
        .iter()
        .map(|x| x.issuer.to_string())
        .collect();

    let account_name = read_tokens()
        .unwrap_or_default()
        .iter()
        .map(|x| x.account_name.to_string())
        .collect();

    let secret = read_tokens()
        .unwrap_or_default()
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
