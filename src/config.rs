use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;
use std::env;

pub struct Config {
    pub jwt_secret: String,
    pub jwt_expires_in: u64, // In seconds
}

impl Config {
    pub fn init() -> Self {
        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your_jwt_secret_key".to_string());
        let jwt_expires_in = env::var("JWT_EXPIRES_IN")
            .unwrap_or_else(|_| "86400".to_string()) // 24 hours
            .parse::<u64>()
            .unwrap_or(86400);

        Self {
            jwt_secret,
            jwt_expires_in,
        }
    }

    pub fn encoding_key(&self) -> EncodingKey {
        EncodingKey::from_secret(self.jwt_secret.as_bytes())
    }

    pub fn decoding_key(&self) -> DecodingKey {
        DecodingKey::from_secret(self.jwt_secret.as_bytes())
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(Config::init);
