extern crate bcrypt;
use bcrypt::{DEFAULT_COST, hash, verify};

pub fn encrypt(s: &str) -> String {
    hash(s, DEFAULT_COST).unwrap()
}

pub fn validate(hashed_str: &str, s: &str) -> bool {
    verify(s, &hashed_str).unwrap()
}