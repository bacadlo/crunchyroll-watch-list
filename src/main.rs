use anyhow::{Result, Context};
use dotenv::dotenv;
use std::env;
use tokio;

mod auth;
mod history;
mod models;

use crate::{
    auth::CrunchyrollClient,
    history::History,
    models::WatchedItem,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    
    let email = env::var("CRUNCHYROLL_EMAIL")
        .context("CRUNCHYROLL_EMAIL not found in environment")?;
    let password = env::var("CRUNCHYROLL_PASSWORD")
        .context("CRUNCHYROLL_PASSWORD not found in environment")?;
    let history_limit = env::var("HISTORY_LIMIT")
        .ok()
        .and_then(|s| s.parse().ok());
    
    let client = CrunchyrollClient::new(&email, &password).await?;
    let history = History::new(&client);

    let items = history.fetch_history(history_limit).await?;
    display_history(&items);

    Ok(())
}

fn display_history(items: &[WatchedItem]) {
    for item in items {
        let progress = format!("{:.1}%", item.progress);
        let status = if item.fully_watched { "Completed" } else { "In Progress" };

        let display = match &item.episode_title {
            Some(episode) => format!(
                "{} - {} (Watched: {}, Progress: {}, Status: {})",
                item.title,
                episode,
                item.date_watched.format("%Y-%m-%d %H:%M"),
                progress,
                status
            ),
            None => format!(
                "{} (Watched: {}, Progress: {}, Status: {})",
                item.title,
                item.date_watched.format("%Y-%m-%d %H:%M"),
                progress,
                status
            ),
        };
        println!("{}", display);
    }
}