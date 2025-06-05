use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHasher, SaltString
    }, Argon2, PasswordHash, PasswordVerifier
};use chrono::{Duration, Utc};
use store::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn generate_jwt_token(email: String) -> Result<String, String>{
    let key = b"secret";

    let tomorrow = Utc::now() + Duration::days(1);

    let claims = Claims{
        sub: email,
        exp: tomorrow.timestamp() as usize,
    };

    match encode(&Header::default(), &claims, &EncodingKey::from_secret(key)) {
        Ok(t) => Ok(t),
        Err(_) => Err(String::from("Error while encoding the ok")),
    }

}


pub fn decode_token(token:&str) -> Result<String, String>{
    // TODO: ADD THIS IN ENV
    let key = b"secret";

    let validation = Validation::default();

    match decode::<Claims>(token, &DecodingKey::from_secret(key), &validation) {
        Ok(c) => Ok(c.claims.sub),
        Err(_e) => Err(String::from("Errrr while decoding the token"))
    }
}


pub fn get_hashed_password(password:&str) -> Result<String, String>{
    
    let salt = SaltString::generate(&mut OsRng);


    let argon2 = Argon2::default();

    let hashed_res = argon2.hash_password(password.as_bytes(), &salt);


    match hashed_res {
        Ok(token) => Ok(token.to_string()),
        Err(_e) => Err(String::from("Error occcurred while hashing password"))
    }
}

pub fn verify_password(actual_hash:&str, password:&String) -> bool{
    let parsed_hash = PasswordHash::new(actual_hash);
    if parsed_hash.is_err(){
        return false;
    }

    let res = Argon2::default().verify_password(password.as_bytes(), &parsed_hash.unwrap()).is_ok();
    res
}