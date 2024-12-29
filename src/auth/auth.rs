use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::{collections::BTreeMap, sync::OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

static KEY: OnceLock<Hmac<Sha256>> = OnceLock::new();

static TOKEN_VALID_DURATION: u64 = 60 * 60 * 24;

fn key() -> &'static Hmac<Sha256> {
    KEY.get_or_init(|| {
        Hmac::new_from_slice(std::env::var("JWT.SECRET").unwrap().as_bytes()).unwrap()
    })
}

pub fn create_jwt_token() -> Result<String, jwt::Error> {
    let mut claims = BTreeMap::new();
    claims.insert("sub", "volo");

    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() + TOKEN_VALID_DURATION;
    let expiration_str = expiration.to_string();
    claims.insert("exp", &expiration_str);

    claims.sign_with_key(key())
}

pub fn verify_jwt_token(token: &String) -> Result<bool, jwt::Error> {
    let claims: BTreeMap<String, String> = token.verify_with_key(key())?;

    let expiration = claims.get("exp").unwrap().parse::<u64>().unwrap();
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    if now > expiration {
        return Ok(false);
    }

    Ok(claims.get("sub").unwrap() == "volo")
}