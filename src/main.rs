mod auth;
mod history;
mod models;

use anyhow::Result;
use dotenv::dotenv; 
use std::env;       
use tracing::{info, Level};

use crate::auth::CrunchyrollClient;
use crate::history::HistoryManager;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    // Load environment variables
    dotenv().ok();
    
    let email = env::var("CRUNCHYROLL_EMAIL")
        .expect("CRUNCHYROLL_EMAIL must be set");
    let password = env::var("CRUNCHYROLL_PASSWORD")
        .expect("CRUNCHYROLL_PASSWORD must be set");

    // Create and authenticate client
    info!("Authenticating with Crunchyroll...");
    let client = CrunchyrollClient::new(&email, &password).await?;

    // Create history manager
    let history_manager = HistoryManager::new(client);

    // Fetch watch history
    info!("Fetching watch history...");
    let history = history_manager.fetch_watch_history().await?;

    // Print watch history
    println!("\nWatch History:");
    for entry in &history {
        println!("\nSeries: {}", entry.series_title);
        println!("Episode: {}", entry.title);
        println!("Progress: {:.2}%", entry.progress);
        println!("Watched at: {}", entry.watched_at);
        println!("Status: {}", entry.status);
        println!("------------------------");
    }

    // Get and print statistics
    info!("Calculating statistics...");
    let stats = history_manager.get_watch_statistics().await?;

    println!("\nWatch Statistics:");
    println!("Total Episodes: {}", stats.total_episodes);
    println!("Completed Episodes: {}", stats.completed_episodes);
    println!("In Progress: {}", stats.in_progress_count);
    println!("Last Watched: {}", stats.last_watched);

    Ok(())
}
