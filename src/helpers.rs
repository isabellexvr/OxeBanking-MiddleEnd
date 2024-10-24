use dotenv::dotenv;
use std::env;

pub fn load_env(key: String) -> Vec<u8> {
    dotenv().ok(); // Load variables from .env

    // Retrieve the secret from the environment, or panic if it's not set
    env::var(key)
        .expect("key sent was not found in .env")
        .into_bytes() // Convert the string to bytes (since jwt requires &[u8])
}
