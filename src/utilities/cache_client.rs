// use redis::Client;
use secrecy::SecretString;

use crate::application::domain::core::Result;

pub struct CacheClient {
    // client: Client,
    host: String,
    credentials: Credentials,
}

struct Credentials {
    username: SecretString,
    password: SecretString,
}

impl CacheClient {
    pub fn generate_key(input: &str) -> String {
        String::from(input)
    }

    pub fn set(&self, key: &str, value: &str) -> Result<()> {
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<()> {
        Ok(())
    }
}

// let redis_client = Client::open(redis_url.as_str())
//     .expect("Failed to create Redis client")
//     .get_connection()
//     .expect("Failed to connect to Redis");
