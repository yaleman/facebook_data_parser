//! Reactions parser

use chrono::{DateTime, TimeZone, Utc};
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::MagicError;

use super::common::{ActivityItem, ActivityType, SearchResult};

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ReactionDetails {
    pub reaction: String,
    pub actor: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ReactionData {
    pub reaction: ReactionDetails,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Reaction {
    pub timestamp: u64,
    pub data: Vec<ReactionData>,
    pub title: String,
}

impl ActivityItem for Reaction {
    fn timestamp(&self) -> DateTime<Utc> {
        // Note: Many reactions have timestamp: 1, which is essentially "unknown"
        // We still parse it but it will show as 1970-01-01
        Utc.timestamp_opt(self.timestamp as i64, 0)
            .single()
            .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())
    }

    fn description(&self) -> String {
        if let Some(reaction_data) = self.data.first() {
            format!(
                "Reaction: {} - {}",
                reaction_data.reaction.reaction, self.title
            )
        } else {
            format!("Reaction: {}", self.title)
        }
    }

    fn activity_type(&self) -> ActivityType {
        ActivityType::Reactions
    }
}

/// Parse a reactions JSON file
pub fn parse_reactions_file(path: &Path) -> Result<Vec<SearchResult>, MagicError> {
    let file = File::open(path)
        .map_err(|e| MagicError::Generic(format!("Failed to open file: {}", e)))?;
    let reader = BufReader::new(file);
    let reactions: Vec<Reaction> = serde_json::from_reader(reader)
        .map_err(|e| MagicError::Generic(format!("Failed to parse reactions JSON: {}", e)))?;

    Ok(reactions.iter().map(SearchResult::new).collect())
}
