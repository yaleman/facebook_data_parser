//! Comments parser

use chrono::{DateTime, TimeZone, Utc};
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::MagicError;

use super::common::{ActivityItem, ActivityType, SearchResult};

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CommentDetails {
    pub timestamp: u64,
    pub comment: String,
    pub author: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CommentData {
    pub comment: CommentDetails,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Comment {
    pub timestamp: u64,
    #[serde(default)]
    pub data: Vec<CommentData>,
    pub title: String,
    #[serde(default)]
    pub attachments: Vec<serde_json::Value>,
}

impl ActivityItem for Comment {
    fn timestamp(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.timestamp as i64, 0)
            .single()
            .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())
    }

    fn description(&self) -> String {
        if let Some(comment_data) = self.data.first() {
            let comment_text = &comment_data.comment.comment;
            if comment_text.chars().count() > 100 {
                let truncated: String = comment_text.chars().take(100).collect();
                format!("Comment: {}... ({})", truncated, self.title)
            } else {
                format!("Comment: {} ({})", comment_text, self.title)
            }
        } else if !self.attachments.is_empty() {
            format!("Comment with attachment: {}", self.title)
        } else {
            format!("Comment: {}", self.title)
        }
    }

    fn activity_type(&self) -> ActivityType {
        ActivityType::Comments
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CommentsFile {
    pub comments_v2: Vec<Comment>,
}

/// Parse a comments JSON file
pub fn parse_comments_file(path: &Path) -> Result<Vec<SearchResult>, MagicError> {
    let file = File::open(path)
        .map_err(|e| MagicError::Generic(format!("Failed to open file: {}", e)))?;
    let reader = BufReader::new(file);
    let comments_file: CommentsFile = serde_json::from_reader(reader)
        .map_err(|e| MagicError::Generic(format!("Failed to parse comments JSON: {}", e)))?;

    Ok(comments_file
        .comments_v2
        .iter()
        .map(SearchResult::new)
        .collect())
}
