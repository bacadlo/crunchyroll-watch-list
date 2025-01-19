use anyhow::Result;
use futures_util::StreamExt;
use crate::{
    auth::CrunchyrollClient,
    models::WatchedItem,
};

pub struct History<'a> {
    client: &'a CrunchyrollClient,
}

impl<'a> History<'a> {
    pub fn new(client: &'a CrunchyrollClient) -> Self {
        Self { client }
    }

    pub async fn fetch_history(&self, limit: Option<usize>) -> Result<Vec<WatchedItem>> {
        let mut history = Vec::new();
        let mut pagination = self.client.client.watch_history();

        if let Some(limit) = limit {
            pagination.page_size(limit as u32);
        }

        while let Some(entry) = pagination.next().await {
            let entry = entry?;
            
            let watched_item = match entry.panel {
                crunchyroll_rs::MediaCollection::Episode(episode) => WatchedItem {
                    title: episode.series_title,
                    episode_title: Some(episode.title),
                    date_watched: entry.date_played,
                    progress: entry.playhead as f32,
                    fully_watched: entry.fully_watched,
                },
                crunchyroll_rs::MediaCollection::Movie(movie) => WatchedItem {
                    title: movie.title,
                    episode_title: None,
                    date_watched: entry.date_played,
                    progress: entry.playhead as f32,
                    fully_watched: entry.fully_watched,
                },
                _ => continue,
            };

            history.push(watched_item);
        }

        Ok(history)
    }
}