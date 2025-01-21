use anyhow::{Result, Context};
use dotenv::dotenv;
use std::env;
use std::fs::File;
use csv::Writer;
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
    save_to_csv(&items)?;

    println!("Successfully exported {} items to history.csv", items.len());
    Ok(())
}

fn save_to_csv(items: &[WatchedItem]) -> Result<()> {
    let file = File::create("history.csv")?;
    let mut writer = Writer::from_writer(file);

    // Write headers
    writer.write_record(&[
        "Title",
        "Episode",
        "Date Watched",
        "Progress",
        "Status",
    ])?;

    // Write data
    for item in items {
        let progress = format!("{:.2}%", item.progress);
        let status = if item.fully_watched { "Completed" } else { "In Progress" };
        
        writer.write_record(&[
            &item.title,
            item.episode_title.as_deref().unwrap_or(""),
            &item.date_watched.format("%Y-%m-%d %H:%M").to_string(),
            &progress,
            status,
        ])?;
    }

    writer.flush()?;
    Ok(())
}