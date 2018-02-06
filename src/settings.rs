extern crate config;

use std::env::args;
use std::path::Path;
use self::config::{Config, File};
use errors::Result;

#[derive(Deserialize)]
pub struct Settings {
    verification_token: String,
    api_token: String,
    data_path: String,
    fallback_url: Option<String>,
}

impl Settings {
    pub fn try_new() -> Result<Self> {
        let mut settings = Config::new();
        settings.merge(
            args()
                .nth(1)
                .map(|a| File::from(Path::new(&a)))
                .unwrap_or_else(|| File::with_name("settings")),
        )?;
        settings.try_into().map_err(|e| e.into())
    }

    pub fn verify(&self, other: &str) -> bool {
        self.verification_token == other
    }

    pub fn token(&self) -> &str {
        &self.api_token
    }

    pub fn data_path(&self) -> &Path {
        Path::new(&self.data_path)
    }

    pub fn fallback_url(&self) -> Option<&str> {
        self.fallback_url.as_ref().map(|s| s.as_ref())
    }
}
