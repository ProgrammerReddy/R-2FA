pub mod totp {
    use totp_rs::{TOTP, Algorithm, Secret};
    use base32::Alphabet;
    use rand::*;

    #[derive(Debug, Default)]
    pub struct Token<'a> {
        pub token: String,
        pub email: String,
        pub issuer: &'a str,
    }

    impl<'a> Token<'a> {
        pub fn new(token: String, email: String, issuer: &'a str) -> Self {
            Self {
                token,
                email,
                issuer,
            }
        }
    }

    #[tauri::command]
    pub fn generate_token() -> Result<(), String> {
        let mut rng = rand::thread_rng();
        let data = rng.gen::<[u8; 21]>();
        let secret = base32::encode(Alphabet::Rfc4648 { padding: false }, &data);

        let totp = TOTP::new(
            Algorithm::SHA1, 
            6, 
            1, 
            30, 
            Secret::Encoded(secret).to_bytes().unwrap()
        ).expect("Failed to generate a token.");

        collect_token(totp);
        Ok(())
    }

    fn collect_token(totp: TOTP) {
        let token = Token::new(totp.get_secret_base32(), "".to_owned(), "");
        let totp_auth = format!("otpauth://totp/{:?}:{:?}?secret={:?}&issuer={:?}", token.issuer, token.email, token.token, token.issuer);
        println!("TOTP Auth: {}", totp_auth);
        println!("TOTP Token: {}", totp);
        let token = totp.generate_current().unwrap();
        println!("TOTP Token: {}", token);

        retrieve_token(totp, totp_auth)
    }

    fn retrieve_token(totp: TOTP, totp_auth: String) {
        totp.check_current(&totp_auth).expect("Failed to retrieve the token.");
        println!("TOTP Token: {}", totp);
    }
} //
