use db::read_tokens;
use crate::{Token, Otp};

use dotenv::dotenv;

pub fn env_as_token() -> Token {
    let _ = dotenv().ok();
    let issuer = read_tokens().iter().map(|x| x.issuer.to_string()).collect();
    let account_name = read_tokens().iter().map(|x| x.account_name.to_string()).collect();
    let secret = read_tokens().iter().map(|x| x.secret.to_string()).collect();

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
