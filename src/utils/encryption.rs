extern crate bcrypt;
use bcrypt::{DEFAULT_COST, hash, verify};
/// Hashes a string value, used for encrypted passwords.
pub fn encrypt(s: &String) -> String {
    hash(&s, DEFAULT_COST).unwrap()
}
/// Checks if a normal string and a hashed string are the same.
/// This is used to check if a user filled in the correct password.
/// When a user writes their password it's not enctyped, but in the DB it is.
pub fn validate(hashed_str: &str, s: &str) -> bool {
    verify(s, &hashed_str).unwrap()
}