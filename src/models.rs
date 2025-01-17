use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Debug)]
pub struct WatchHistoryEntry {
    pub episode_id: String,
    pub title: String,
    pub watched_at: DateTime<Utc>,
    pub progress: f64,
    pub status: WatchStatus,
    pub series_title: String,
}

#[derive(Debug)]
pub enum WatchStatus {
    Completed,
    InProgress,
    NotStarted,
}

impl fmt::Display for WatchStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WatchStatus::Completed => write!(f, "completed"),
            WatchStatus::InProgress => write!(f, "in_progress"),
            WatchStatus::NotStarted => write!(f, "not_started"),
        }
    }
}

#[derive(Debug)]
pub struct WatchStatistics {
    pub total_episodes: usize,
    pub completed_episodes: usize,
    pub in_progress_count: usize,
    pub last_watched: DateTime<Utc>,
}