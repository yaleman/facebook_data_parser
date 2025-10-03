//! Common types and traits for all activity types

use chrono::{DateTime, Utc};
use std::fmt::Display;

/// Trait for any Facebook activity item that has a timestamp
pub trait ActivityItem {
    /// Get the timestamp of this activity
    fn timestamp(&self) -> DateTime<Utc>;

    /// Get a display-friendly description of this activity
    fn description(&self) -> String;

    /// Get the type of this activity
    fn activity_type(&self) -> ActivityType;
}

/// All supported Facebook activity types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActivityType {
    Messages,
    Posts,
    Comments,
    Reactions,
    Events,
    Groups,
    Friends,
}

impl Display for ActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActivityType::Messages => write!(f, "messages"),
            ActivityType::Posts => write!(f, "posts"),
            ActivityType::Comments => write!(f, "comments"),
            ActivityType::Reactions => write!(f, "reactions"),
            ActivityType::Events => write!(f, "events"),
            ActivityType::Groups => write!(f, "groups"),
            ActivityType::Friends => write!(f, "friends"),
        }
    }
}

impl std::str::FromStr for ActivityType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "messages" => Ok(ActivityType::Messages),
            "posts" => Ok(ActivityType::Posts),
            "comments" => Ok(ActivityType::Comments),
            "reactions" => Ok(ActivityType::Reactions),
            "events" => Ok(ActivityType::Events),
            "groups" => Ok(ActivityType::Groups),
            "friends" => Ok(ActivityType::Friends),
            _ => Err(format!("Unknown activity type: {}", s)),
        }
    }
}

/// Unified search result wrapper
#[derive(Debug)]
pub struct SearchResult {
    pub timestamp: DateTime<Utc>,
    pub activity_type: ActivityType,
    pub description: String,
}

impl SearchResult {
    pub fn new<T: ActivityItem>(item: &T) -> Self {
        Self {
            timestamp: item.timestamp(),
            activity_type: item.activity_type(),
            description: item.description(),
        }
    }
}

impl Display for SearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} - {}",
            self.timestamp.format("%Y-%m-%d %H:%M:%S"),
            self.activity_type,
            self.description
        )
    }
}
