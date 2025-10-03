//! Posts parser

use chrono::{DateTime, TimeZone, Utc};
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::MagicError;

use super::common::{ActivityItem, ActivityType, SearchResult};

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PostData {
    #[serde(default)]
    pub post: Option<String>,
    #[serde(default)]
    pub update_timestamp: Option<u64>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Post {
    pub timestamp: u64,
    pub data: Vec<PostData>,
}

impl ActivityItem for Post {
    fn timestamp(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.timestamp as i64, 0)
            .single()
            .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())
    }

    fn description(&self) -> String {
        let content = self
            .data
            .iter()
            .filter_map(|d| d.post.as_ref())
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ");

        if content.is_empty() {
            "Post (no text content)".to_string()
        } else {
            // Limit to first 100 chars
            if content.chars().count() > 100 {
                let truncated: String = content.chars().take(100).collect();
                format!("Post: {}...", truncated)
            } else {
                format!("Post: {}", content)
            }
        }
    }

    fn activity_type(&self) -> ActivityType {
        ActivityType::Posts
    }
}

/// Parse a posts JSON file
pub fn parse_posts_file(path: &Path) -> Result<Vec<SearchResult>, MagicError> {
    let file = File::open(path)
        .map_err(|e| MagicError::Generic(format!("Failed to open file: {}", e)))?;
    let reader = BufReader::new(file);
    let posts: Vec<Post> = serde_json::from_reader(reader)
        .map_err(|e| MagicError::Generic(format!("Failed to parse posts JSON: {}", e)))?;

    Ok(posts.iter().map(SearchResult::new).collect())
}
