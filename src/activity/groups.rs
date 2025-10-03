//! Groups parser

use chrono::{DateTime, TimeZone, Utc};
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::MagicError;

use super::common::{ActivityItem, ActivityType, SearchResult};

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GroupAdmin {
    pub name: String,
    pub timestamp: u64,
}

impl ActivityItem for GroupAdmin {
    fn timestamp(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.timestamp as i64, 0)
            .single()
            .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())
    }

    fn description(&self) -> String {
        format!("Group Admin: {}", self.name)
    }

    fn activity_type(&self) -> ActivityType {
        ActivityType::Groups
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GroupsAdminFile {
    pub groups_admined_v2: Vec<GroupAdmin>,
}

/// Parse a groups admin JSON file
pub fn parse_groups_admin_file(path: &Path) -> Result<Vec<SearchResult>, MagicError> {
    let file = File::open(path)
        .map_err(|e| MagicError::Generic(format!("Failed to open file: {}", e)))?;
    let reader = BufReader::new(file);
    let groups_file: GroupsAdminFile = serde_json::from_reader(reader)
        .map_err(|e| MagicError::Generic(format!("Failed to parse groups JSON: {}", e)))?;

    Ok(groups_file
        .groups_admined_v2
        .iter()
        .map(SearchResult::new)
        .collect())
}
