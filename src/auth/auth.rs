use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::{collections::BTreeMap, sync::OnceLock};

static KEY: OnceLock<Hmac<Sha256>> = OnceLock::new();

fn key() -> &'static Hmac<Sha256> {
    KEY.get_or_init(|| {
        Hmac::new_from_slice(std::env::var("JWT.SECRET").unwrap().as_bytes()).unwrap()
    })
}

pub fn create_jwt_token() -> Result<String, jwt::Error> {
    let mut claims = BTreeMap::new();
    claims.insert("sub", "volo");
    claims.insert("exp", "3600");

    claims.sign_with_key(key())
}

pub fn verify_jwt_token(token: String) -> Result<bool, jwt::Error> {
    let claims: BTreeMap<String, String> = token.verify_with_key(key())?;
    Ok(claims.get("sub").unwrap() == "volo")
}