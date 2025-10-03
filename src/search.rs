//! Search functionality across all Facebook data types

use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use std::collections::HashSet;
use std::path::Path;

use crate::activity::comments::parse_comments_file;
use crate::activity::common::{ActivityType, SearchResult};
use crate::activity::events::parse_events_file;
use crate::activity::groups::parse_groups_admin_file;
use crate::activity::messages::MessageFileParser;
use crate::activity::posts::parse_posts_file;
use crate::activity::reactions::parse_reactions_file;
use crate::{MagicError, BASE_PATH};

pub struct SearchOptions {
    pub earliest: Option<DateTime<Utc>>,
    pub latest: Option<DateTime<Utc>>,
    pub types: Option<HashSet<ActivityType>>,
    pub show_paths: bool,
    pub has_attachments: bool,
}

impl SearchOptions {
    pub fn new(
        earliest_str: Option<String>,
        latest_str: Option<String>,
        types_str: Option<String>,
        show_paths: bool,
        has_attachments: bool,
    ) -> Result<Self, MagicError> {
        let earliest = if let Some(date_str) = earliest_str {
            Some(parse_date(&date_str)?)
        } else {
            None
        };

        let latest = if let Some(date_str) = latest_str {
            Some(parse_date(&date_str)?)
        } else {
            None
        };

        let types = if let Some(types_str) = types_str {
            let mut type_set = HashSet::new();
            for type_str in types_str.split(',') {
                let activity_type = type_str
                    .trim()
                    .parse::<ActivityType>()
                    .map_err(MagicError::Generic)?;
                type_set.insert(activity_type);
            }
            Some(type_set)
        } else {
            None
        };

        Ok(Self {
            earliest,
            latest,
            types,
            show_paths,
            has_attachments,
        })
    }

    fn matches(&self, result: &SearchResult) -> bool {
        // Check date range
        if let Some(earliest) = self.earliest {
            if result.timestamp < earliest {
                return false;
            }
        }

        if let Some(latest) = self.latest {
            if result.timestamp > latest {
                return false;
            }
        }

        // Check type filter
        if let Some(ref types) = self.types {
            if !types.contains(&result.activity_type) {
                return false;
            }
        }

        true
    }
}

fn parse_date(date_str: &str) -> Result<DateTime<Utc>, MagicError> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|e| MagicError::Generic(format!("Invalid date format '{}': {}", date_str, e)))
        .map(|date| {
            Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0).unwrap())
        })
}

pub fn search(options: SearchOptions) -> Result<Vec<SearchResult>, MagicError> {
    let mut all_results = Vec::new();

    // Determine which types to search
    let search_posts = options
        .types
        .as_ref()
        .map(|t| t.contains(&ActivityType::Posts))
        .unwrap_or(true);
    let search_comments = options
        .types
        .as_ref()
        .map(|t| t.contains(&ActivityType::Comments))
        .unwrap_or(true);
    let search_reactions = options
        .types
        .as_ref()
        .map(|t| t.contains(&ActivityType::Reactions))
        .unwrap_or(true);
    let search_events = options
        .types
        .as_ref()
        .map(|t| t.contains(&ActivityType::Events))
        .unwrap_or(true);
    let search_groups = options
        .types
        .as_ref()
        .map(|t| t.contains(&ActivityType::Groups))
        .unwrap_or(true);
    let search_messages = options
        .types
        .as_ref()
        .map(|t| t.contains(&ActivityType::Messages))
        .unwrap_or(true);

    // Search posts
    if search_posts {
        println!("Searching posts...");
        let posts_path = format!("{}/your_activity_across_facebook/posts", BASE_PATH);
        match search_posts_directory(&posts_path) {
            Ok(results) => all_results.extend(results),
            Err(e) => eprintln!("Error searching posts: {:?}", e),
        }
    }

    // Search comments
    if search_comments {
        println!("Searching comments...");
        let comments_path = format!(
            "{}/your_activity_across_facebook/comments_and_reactions/comments.json",
            BASE_PATH
        );
        if Path::new(&comments_path).exists() {
            match parse_comments_file(Path::new(&comments_path)) {
                Ok(results) => all_results.extend(results),
                Err(e) => eprintln!("Error parsing comments: {:?}", e),
            }
        }
    }

    // Search reactions
    if search_reactions {
        println!("Searching reactions...");
        let reactions_dir = format!(
            "{}/your_activity_across_facebook/comments_and_reactions",
            BASE_PATH
        );
        if let Ok(results) = search_reactions_directory(&reactions_dir) {
            all_results.extend(results);
        }
    }

    // Search events
    if search_events {
        println!("Searching events...");
        let events_path = format!(
            "{}/your_activity_across_facebook/events/your_events.json",
            BASE_PATH
        );
        if Path::new(&events_path).exists() {
            if let Ok(results) = parse_events_file(Path::new(&events_path)) {
                all_results.extend(results);
            }
        }
    }

    // Search groups
    if search_groups {
        println!("Searching groups...");
        let groups_path = format!(
            "{}/your_activity_across_facebook/groups/your_groups.json",
            BASE_PATH
        );
        if Path::new(&groups_path).exists() {
            if let Ok(results) = parse_groups_admin_file(Path::new(&groups_path)) {
                all_results.extend(results);
            }
        }
    }

    // Search messages
    if search_messages {
        println!("Searching messages...");
        let messages_dir = format!("{}/your_activity_across_facebook/messages", BASE_PATH);
        match search_messages_directory(&messages_dir, &options) {
            Ok(results) => all_results.extend(results),
            Err(e) => eprintln!("Error searching messages: {:?}", e),
        }
    }

    // Filter results by search options
    let filtered_results: Vec<SearchResult> = all_results
        .into_iter()
        .filter(|r| options.matches(r))
        .collect();

    // Sort by timestamp
    let mut sorted_results = filtered_results;
    sorted_results.sort_by_key(|r| r.timestamp);

    println!(
        "\nFound {} matching results\n",
        sorted_results.len()
    );

    Ok(sorted_results)
}

fn search_posts_directory(dir_path: &str) -> Result<Vec<SearchResult>, MagicError> {
    let mut results = Vec::new();

    let pattern = format!("{}/*.json", dir_path);
    for entry in glob::glob(&pattern).map_err(|e| MagicError::Generic(e.to_string()))? {
        match entry {
            Ok(path) => {
                if let Ok(file_results) = parse_posts_file(&path) {
                    results.extend(file_results);
                }
            }
            Err(e) => eprintln!("Error reading path: {:?}", e),
        }
    }

    Ok(results)
}

fn search_reactions_directory(dir_path: &str) -> Result<Vec<SearchResult>, MagicError> {
    let mut results = Vec::new();

    let pattern = format!("{}/likes_and_reactions_*.json", dir_path);
    for entry in glob::glob(&pattern).map_err(|e| MagicError::Generic(e.to_string()))? {
        match entry {
            Ok(path) => {
                if let Ok(file_results) = parse_reactions_file(&path) {
                    results.extend(file_results);
                }
            }
            Err(e) => eprintln!("Error reading path: {:?}", e),
        }
    }

    Ok(results)
}

fn is_message_file(path: &Path) -> bool {
    let valid_filename = regex::Regex::new(r"\/message_[\d]+\.json$").unwrap();
    valid_filename.is_match(path.to_str().unwrap())
}

fn message_has_attachments(message: &crate::activity::messages::Message) -> bool {
    message.photos.is_some()
        || message.videos.is_some()
        || message.files.is_some()
        || message.sticker.is_some()
}

fn format_message_with_paths(message: &crate::activity::messages::Message) -> String {
    use crate::activity::common::ActivityItem;

    let mut description = message.description();
    let mut paths = Vec::new();

    if let Some(photos) = &message.photos {
        for photo in photos {
            paths.push(format!("  📷 {}", photo.uri));
        }
    }

    if let Some(videos) = &message.videos {
        for video in videos {
            paths.push(format!("  🎥 {}", video.uri));
        }
    }

    if let Some(files) = &message.files {
        for file in files {
            paths.push(format!("  📄 {}", file.uri));
        }
    }

    if let Some(sticker) = &message.sticker {
        paths.push(format!("  🎨 {}", sticker.uri));
    }

    if !paths.is_empty() {
        description.push('\n');
        description.push_str(&paths.join("\n"));
    }

    description
}

fn search_messages_directory(
    dir_path: &str,
    options: &SearchOptions,
) -> Result<Vec<SearchResult>, MagicError> {
    use crate::activity::common::ActivityItem;

    let mut results = Vec::new();

    let pattern = format!("{}/**/*.json", dir_path);
    for entry in glob::glob(&pattern).map_err(|e| MagicError::Generic(e.to_string()))? {
        match entry {
            Ok(path) => {
                if !is_message_file(&path) {
                    continue;
                }

                let message_file = MessageFileParser::try_from(&path)?;
                for message in &message_file.messages {
                    // Filter by attachments if requested
                    if options.has_attachments && !message_has_attachments(message) {
                        continue;
                    }

                    // Create search result with custom description if showing paths
                    let result = if options.show_paths && message_has_attachments(message) {
                        SearchResult {
                            timestamp: message.timestamp(),
                            activity_type: message.activity_type(),
                            description: format_message_with_paths(message),
                        }
                    } else {
                        SearchResult::new(message)
                    };

                    results.push(result);
                }
            }
            Err(e) => eprintln!("Error reading path: {:?}", e),
        }
    }

    Ok(results)
}
