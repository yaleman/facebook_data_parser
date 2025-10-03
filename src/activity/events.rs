//! Events parser

use chrono::{DateTime, TimeZone, Utc};
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::MagicError;

use super::common::{ActivityItem, ActivityType, SearchResult};

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventPlace {
    pub name: String,
    #[serde(default)]
    pub coordinate: Option<EventCoordinate>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventCoordinate {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Event {
    pub name: String,
    pub start_timestamp: u64,
    #[serde(default)]
    pub end_timestamp: Option<u64>,
    #[serde(default)]
    pub place: Option<EventPlace>,
    #[serde(default)]
    pub description: Option<String>,
    pub create_timestamp: u64,
}

impl ActivityItem for Event {
    fn timestamp(&self) -> DateTime<Utc> {
        // Use start_timestamp as the primary timestamp
        Utc.timestamp_opt(self.start_timestamp as i64, 0)
            .single()
            .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())
    }

    fn description(&self) -> String {
        let place_str = self
            .place
            .as_ref()
            .map(|p| format!(" at {}", p.name))
            .unwrap_or_default();

        format!("Event: {}{}", self.name, place_str)
    }

    fn activity_type(&self) -> ActivityType {
        ActivityType::Events
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventsFile {
    pub your_events_v2: Vec<Event>,
}

/// Parse an events JSON file
pub fn parse_events_file(path: &Path) -> Result<Vec<SearchResult>, MagicError> {
    let file = File::open(path)
        .map_err(|e| MagicError::Generic(format!("Failed to open file: {}", e)))?;
    let reader = BufReader::new(file);
    let events_file: EventsFile = serde_json::from_reader(reader)
        .map_err(|e| MagicError::Generic(format!("Failed to parse events JSON: {}", e)))?;

    Ok(events_file
        .your_events_v2
        .iter()
        .map(SearchResult::new)
        .collect())
}
