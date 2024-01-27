use std::path::PathBuf;

use enum_iterator::Sequence;

use crate::{Skippable, BASE_PATH};

pub mod messages;

/// Activity parser
///
///

static PARENT_FOLDER: &str = "your_activity_across_facebook";

#[derive(Debug, PartialEq, Sequence)]
pub enum ActivityTypes {
    BugBounty,
    Messages,
}

// impl ActivityTypes {}

impl Skippable for ActivityTypes {
    fn path(&self) -> PathBuf {
        match self {
            ActivityTypes::BugBounty => {
                PathBuf::from(&format!("{}/{}/bug_bounty", BASE_PATH, PARENT_FOLDER))
            }
            ActivityTypes::Messages => {
                PathBuf::from(&format!("{}/{}/messages", BASE_PATH, PARENT_FOLDER))
            }
        }
    }

    fn skippable(&self) -> bool {
        self.path().join("no-data.txt").exists()
    }
}
