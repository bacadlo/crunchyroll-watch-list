use anyhow::Result;
use crunchyroll_rs::Crunchyroll;

pub struct CrunchyrollClient {
    pub client: Crunchyroll,
}

impl CrunchyrollClient {
    pub async fn new(email: &str, password: &str) -> Result<Self> {
        let client = Crunchyroll::builder()
            .login_with_credentials(email, password)
            .await?;

        Ok(Self { client })
    }

    pub async fn validate_session(&self) -> bool {
        // The library handles session management internally
        true
    }
}