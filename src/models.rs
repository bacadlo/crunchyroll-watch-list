use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedItem {
    pub title: String,
    pub episode_title: Option<String>,
    pub date_watched: DateTime<Utc>,
    pub progress: f32,
    pub fully_watched: bool,
}