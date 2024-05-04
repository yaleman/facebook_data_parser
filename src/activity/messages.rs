//!
//!  Messages related things
//!
use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use std::net::IpAddr;
use std::path::{Path, PathBuf};

use rayon::prelude::*;
use regex::Regex;
use serde::Deserialize;

use crate::{ActivityMessages, MagicError, BASE_PATH};

pub struct MessageBox {
    pub filepath: String,
    pub filename: String,
    pub participants: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MessageParticipant {
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MessageMagicWord {
    pub magic_word: String,

    #[serde(
        with = "chrono::serde::ts_milliseconds_option",
        default = "default_none_dt"
    )]
    pub creation_timestamp_ms: Option<DateTime<Utc>>,
    // pub creation_timestamp_ms: u64,
    pub animation_emoji: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MessageJoinableMode {
    pub mode: usize,
    pub link: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MessageFileParser {
    pub participants: Vec<MessageParticipant>,
    pub messages: Vec<Message>,
    pub title: String,
    pub is_still_participant: bool,
    pub thread_path: String,
    pub magic_words: Vec<MessageMagicWord>,
    pub image: Option<MessagePhoto>,
    pub joinable_mode: Option<MessageJoinableMode>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MessageShare {
    pub link: Option<String>,
    pub share_text: Option<String>,
    pub is_geoblocked_for_viewer: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MessageMedia {
    pub uri: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds_option", default = "default_none_dt")]
    pub creation_timestamp: Option<DateTime<Utc>>,
    pub share_text: Option<String>,
    pub is_geoblocked_for_viewer: Option<bool>,
}

use chrono::{DateTime, Utc};

fn default_none_dt() -> Option<DateTime<Utc>> {
    None
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MessagePhoto {
    pub uri: String,
    #[serde(with = "chrono::serde::ts_seconds_option", default = "default_none_dt")]
    pub creation_timestamp: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MessageReaction {
    pub reaction: String,
    pub actor: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MessageVideo {
    pub uri: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub creation_timestamp: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MessageAiSticker {
    pub input: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MessageSticker {
    pub uri: String,
    pub ai_stickers: Vec<MessageAiSticker>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MessageFile {
    pub uri: String,
    #[serde(with = "chrono::serde::ts_seconds_option", default = "default_none_dt")]
    pub creation_timestamp: Option<DateTime<Utc>>,
    pub title: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Message {
    pub sender_name: String,
    pub is_unsent: Option<bool>,
    pub timestamp_ms: u64,
    pub content: Option<String>,
    pub share: Option<MessageShare>,
    pub videos: Option<Vec<MessageVideo>>,
    pub reactions: Option<Vec<MessageReaction>>,
    pub photos: Option<Vec<MessagePhoto>>,
    pub gifs: Option<Vec<MessagePhoto>>,
    pub is_geoblocked_for_viewer: bool,
    pub call_duration: Option<u64>,
    pub sticker: Option<MessageSticker>,
    pub files: Option<Vec<MessageFile>>,
    pub audio_files: Option<Vec<MessageMedia>>,
    pub ip: Option<IpAddr>,
    pub missed: Option<bool>,
}

impl TryFrom<&PathBuf> for MessageFileParser {
    type Error = MagicError;
    fn try_from(path: &PathBuf) -> Result<Self, MagicError> {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let data: Result<MessageFileParser, MagicError> =
            serde_json::from_reader(reader).map_err(|err| MagicError::Generic(err.to_string()));
        data
    }
}

fn is_message_file(path: &Path) -> bool {
    let valid_filename = regex::Regex::new(r"\/message_[\d]+\.json$").unwrap();
    valid_filename.is_match(path.to_str().unwrap())
}

pub fn select_message_folder() -> PathBuf {
    let mut folders = Vec::new();

    for entry in glob::glob(&format!(
        "{}/your_activity_across_facebook/messages/**/*.json",
        BASE_PATH
    ))
    .expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => {
                if is_message_file(&path) {
                    let parent_folder = path.clone();
                    let parent_folder = parent_folder.parent().unwrap();
                    if parent_folder.is_dir() {
                        if !folders.contains(&parent_folder.to_path_buf()) {
                            folders.push(parent_folder.to_owned());
                        }
                    } else {
                        panic!("Uh, {:?} is a parent but is not a dir?", parent_folder);
                    }
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
    println!("Found {} folders", folders.len());
    let folders_display: Vec<String> = folders.iter().map(|f| f.display().to_string()).collect();

    let res = dialoguer::FuzzySelect::new()
        .items(&folders_display)
        .with_prompt("Select a folder")
        .interact()
        .unwrap();
    folders[res].clone()
}

pub fn get_all_messages(folder: &Path) -> Result<Vec<Message>, MagicError> {
    let mut parsed_filecount = 0;
    let mut messages = Vec::new();
    for entry in
        glob::glob(&format!("{}/**/*.json", folder.display())).expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => {
                if !is_message_file(&path) {
                    continue;
                } else {
                    // eprintln!("Trying {}", path.display());
                    let parsed = MessageFileParser::try_from(&path)?;
                    messages.extend(parsed.messages);
                    parsed_filecount += 1;
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
    println!(
        "Parsed {} files, found {} messages",
        parsed_filecount,
        messages.len()
    );
    Ok(messages)
}

pub fn reorg_videos(msg: ActivityMessages) -> Result<(), MagicError> {
    let folder = match msg.target_folder {
        Some(folder) => PathBuf::from(folder),
        None => select_message_folder(),
    };

    let messages = get_all_messages(&folder)?;
    let username = folder.iter().last().unwrap().to_str().unwrap();

    messages.par_iter().for_each(|msg| {
        // println!("{:?}", msg);
        if let Some(videos) = &msg.videos {
            for video in videos {
                // println!("Photo: {:?}", photo);

                let filepath = PathBuf::from(format!("{}/{}", BASE_PATH, video.uri));

                let datepath = PathBuf::from(format!(
                    "output/{}/{}",
                    username,
                    video.creation_timestamp.format("%Y/%m")
                ));
                if !datepath.exists() {
                    std::fs::create_dir_all(&datepath).unwrap();
                }
                let timestamp_filebit = video.creation_timestamp.format("%Y-%m-%d-%H-%M-%S");

                let new_filename = format!(
                    "{}/{}-{}",
                    datepath.display(),
                    timestamp_filebit,
                    filepath.file_name().unwrap().to_str().unwrap()
                );
                println!("new_filename {}", new_filename);
                // copy the file
                std::fs::copy(&filepath, &new_filename).unwrap();
            }
        }
    });

    Ok(())
}

pub fn reorg_images(msg: ActivityMessages) -> Result<(), MagicError> {
    // println!("Messages: {:?}", msg);
    let folder = match msg.target_folder {
        Some(folder) => PathBuf::from(folder),
        None => select_message_folder(),
    };
    println!("Target folder: {}", folder.display());
    let messages = get_all_messages(&folder)?;

    let username = folder.iter().last().unwrap().to_str().unwrap();

    messages.par_iter().for_each(|msg| {
        // println!("{:?}", msg);
        if let Some(photos) = &msg.photos {
            for photo in photos {
                // println!("Photo: {:?}", photo);
                match photo.creation_timestamp {
                    Some(timestamp) => {
                        let filepath = PathBuf::from(format!("{}/{}", BASE_PATH, photo.uri));

                        let datepath = PathBuf::from(format!(
                            "output/{}/{}",
                            username,
                            timestamp.format("%Y/%m")
                        ));
                        if !datepath.exists() {
                            std::fs::create_dir_all(&datepath).unwrap();
                        }
                        let timestamp_filebit = timestamp.format("%Y-%m-%d-%H-%M-%S");

                        let new_filename = format!(
                            "{}/{}-{}",
                            datepath.display(),
                            timestamp_filebit,
                            filepath.file_name().unwrap().to_str().unwrap()
                        );
                        println!("new_filename {}", new_filename);
                        // copy the file
                        std::fs::copy(&filepath, &new_filename).unwrap();
                        // println!("{}", new_filenamehoto);
                    }
                    None => todo!("handle {:?}", photo),
                }
            }
        }
    });

    Ok(())
}

#[derive(Default, Debug)]
struct SearchTerms {
    pub earliest: Option<DateTime<Utc>>,
    pub latest: Option<DateTime<Utc>>,
    pub string: Option<String>,
    pub regex: Option<Regex>,
}

enum SearchMenu {
    SetEarliest,
    ClearEarliest,
    SetLatest,
    ClearLatest,
    SetString,
    ClearString,
    SetRegex,
    ClearRegex,
    Run,
    Quit,
}

impl AsRef<str> for SearchMenu {
    fn as_ref(&self) -> &str {
        match self {
            SearchMenu::SetEarliest => "Set Earliest Date",
            SearchMenu::ClearEarliest => "Clear Earliest Date",
            SearchMenu::SetLatest => "Set Latest Date",
            SearchMenu::ClearLatest => "Clear Latest Date",
            SearchMenu::SetString => "Set Search String",
            SearchMenu::ClearString => "Clear Search String",
            SearchMenu::SetRegex => "Set Search Regex",
            SearchMenu::ClearRegex => "Clear Search Regex",
            SearchMenu::Quit => "Quit",
            SearchMenu::Run => "Run",
        }
    }
}

impl From<usize> for SearchMenu {
    fn from(value: usize) -> Self {
        match value {
            0 => SearchMenu::SetEarliest,
            1 => SearchMenu::ClearEarliest,
            2 => SearchMenu::SetLatest,
            3 => SearchMenu::ClearLatest,
            4 => SearchMenu::SetString,
            5 => SearchMenu::ClearString,
            6 => SearchMenu::SetRegex,
            7 => SearchMenu::ClearRegex,
            8 => SearchMenu::Run,
            9 => SearchMenu::Quit,
            _ => panic!("Invalid value"),
        }
    }
}

impl Display for SearchMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

pub fn search_messages(path: Option<PathBuf>) -> Result<(), MagicError> {
    let path = match path {
        Some(path) => path,
        None => select_message_folder(),
    };
    println!("Loading messages from target folder: {}", path.display());
    let messages = get_all_messages(&path)?;

    let mut searchterms = SearchTerms::default();
    loop {
        eprintln!("#################################");
        eprintln!("Search terms:");
        eprintln!("{:#?}", &searchterms);
        eprintln!("#################################");

        let selected: SearchMenu = dialoguer::Select::new()
            .with_prompt("Select search type")
            .items(
                &[
                    SearchMenu::SetEarliest,
                    SearchMenu::ClearEarliest,
                    SearchMenu::SetLatest,
                    SearchMenu::ClearLatest,
                    SearchMenu::SetString,
                    SearchMenu::ClearString,
                    SearchMenu::SetRegex,
                    SearchMenu::ClearRegex,
                    SearchMenu::Run,
                    SearchMenu::Quit,
                ]
                .map(|x| x.to_string()),
            )
            .interact()
            .unwrap()
            .into();
        println!("Selected: {}", selected);

        match selected {
            SearchMenu::SetEarliest => {
                let earliest = dialoguer::Input::<String>::new()
                    .with_prompt("Enter earliest date")
                    .interact()
                    .unwrap();
                let earliest = match DateTime::parse_from_rfc3339(&earliest) {
                    Ok(dt) => dt.with_timezone(&Utc),
                    Err(e) => {
                        eprintln!("Failed to parse date: {:?}", e);
                        continue;
                    }
                };
                searchterms.earliest = Some(earliest);
            }
            SearchMenu::ClearEarliest => searchterms.earliest = None,
            SearchMenu::SetLatest => {
                let latest = dialoguer::Input::<String>::new()
                    .with_prompt("Enter latest date")
                    .interact()
                    .unwrap();
                let latest = match DateTime::parse_from_rfc3339(&latest) {
                    Ok(dt) => dt.with_timezone(&Utc),
                    Err(e) => {
                        eprintln!("Failed to parse date: {:?}", e);
                        continue;
                    }
                };
                searchterms.latest = Some(latest);
            }
            SearchMenu::ClearLatest => searchterms.latest = None,
            SearchMenu::SetString => {
                let string = dialoguer::Input::<String>::new()
                    .with_prompt("Enter search string")
                    .interact()
                    .unwrap();
                if string.trim().is_empty() {
                    eprintln!("String cannot be empty");
                    continue;
                }
                searchterms.string = Some(string);
            }
            SearchMenu::ClearString => searchterms.string = None,
            SearchMenu::SetRegex => {
                let regex = dialoguer::Input::<String>::new()
                    .with_prompt("Enter search regex")
                    .interact()
                    .unwrap();
                if regex.trim().is_empty() {
                    eprintln!("Regex cannot be empty");
                    continue;
                }
                match regex::Regex::try_from(regex) {
                    Err(err) => {
                        eprintln!("Failed to parse regex: {:?}", err);
                        continue;
                    }
                    Ok(regex) => {
                        searchterms.regex = Some(regex);
                    }
                }
            }
            SearchMenu::ClearRegex => searchterms.regex = None,
            SearchMenu::Quit => return Ok(()),
            SearchMenu::Run => {
                println!("Running search");

                break;
            }
        }
    }
    let regex = searchterms.regex.clone();

    messages.into_iter().for_each(|msg| {
        if let Some(earliest) = searchterms.earliest {
            if msg.timestamp_ms < earliest.timestamp_millis() as u64 {
                return;
            }
        }

        if let Some(latest) = searchterms.latest {
            if msg.timestamp_ms > latest.timestamp_millis() as u64 {
                return;
            }
        }

        // filter on string

        if let Some(string) = &searchterms.string {
            if !msg
                .content
                .as_ref()
                .unwrap_or(&"".to_string())
                .contains(string)
            {
                return;
            }
        }

        // filter on regex
        if let Some(regex) = &regex {
            if !regex.is_match(&msg.content.clone().unwrap_or("".to_string())) {
                return;
            }
        }
        println!("{:?}", &msg)
    });
    Ok(())
}

pub fn list_files(msg: ActivityMessages) -> Result<(), MagicError> {
    let folder = match msg.target_folder {
        Some(folder) => PathBuf::from(folder),
        None => select_message_folder(),
    };
    println!("Target folder: {}", folder.display());
    let messages = get_all_messages(&folder)?;

    // let username = folder.iter().last().unwrap().to_str().unwrap();

    messages.par_iter().for_each(|msg| {
        if let Some(files) = msg.files.clone() {
            for file in files {
                println!("File: {:?}", file);
            }
        }
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::BASE_PATH;

    use super::{is_message_file, MessageFileParser};

    #[test]
    fn test_messagefileparser() {
        let mut parsed_filecount = 0;

        for entry in glob::glob(&format!(
            "{}/your_activity_across_facebook/messages/**/*.json",
            BASE_PATH
        ))
        .expect("Failed to read glob pattern")
        {
            match entry {
                Ok(path) => {
                    if !is_message_file(&path) {
                        continue;
                    } else {
                        eprintln!("Trying {}", path.display());
                        let _parsed = MessageFileParser::try_from(&path).unwrap();
                        parsed_filecount += 1;
                    }
                }
                Err(e) => eprintln!("{:?}", e),
            }
        }
        println!("Parsed {} files", parsed_filecount);
    }
}
