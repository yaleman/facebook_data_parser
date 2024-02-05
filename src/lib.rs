use std::path::{Path, PathBuf};
use std::process::exit;
use std::str::FromStr;

use clap::{Args, Subcommand};
use enum_iterator::Sequence;

pub mod activity;

#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliCommand {
    // #[clap(name = "activity", about = "Do something with activity data")]
    #[clap(subcommand)]
    pub command: CliCommands,
}

#[derive(Subcommand, Debug)]
pub enum CliCommands {
    Activity {
        #[clap(subcommand)]
        command: ActivityActivity,
    },
}

#[derive(Subcommand, Debug)]
pub enum ActivityActivity {
    Messages(ActivityMessages),
}

#[derive(Subcommand, Debug, Clone)]
pub enum ActivityMessagesSubCommand {
    ReorgImages,
    ReorgVideos,
    ListFiles,
}

#[derive(Args, Debug)]
pub struct ActivityMessages {
    #[clap(subcommand)]
    pub command: ActivityMessagesSubCommand,
    #[clap(short, long)]
    pub target_folder: Option<String>,
}

pub(crate) static BASE_PATH: &str = "data";

#[derive(Debug, PartialEq, Sequence)]
pub enum Folders {
    AdsInformation,
    AppsAndWebsitesOffOfFacebook,
    Connections,
    LoggedInformation,
    PersonalInformation,
    Preferences,
    SecurityAndLoginInformation,
    YourActivityAcrossFacebook,
}

impl Folders {
    pub fn path(&self) -> &str {
        match self {
            Folders::AdsInformation => "ads_information",
            Folders::AppsAndWebsitesOffOfFacebook => "apps_and_websites_off_of_facebook",
            Folders::Connections => "connections",
            Folders::LoggedInformation => "logged_information",
            Folders::PersonalInformation => "personal_information",
            Folders::Preferences => "preferences",
            Folders::SecurityAndLoginInformation => "security_and_login_information",
            Folders::YourActivityAcrossFacebook => "your_activity_across_facebook",
        }
    }
}

#[derive(Debug)]
pub enum MagicError {
    Generic(String),
    Skippable,
}

impl FromStr for MagicError {
    type Err = MagicError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MagicError::Generic(s.to_string()))
    }
}

/// Checks the various folders are where we think they are
pub fn folder_checks() {
    let path = "data/";
    println!("Checking all required folders exist in {}", path);
    let path = Path::new(path);
    for folder in enum_iterator::all::<Folders>() {
        let folder_path = path.join(folder.path());
        if !folder_path.exists() {
            eprintln!("{} does not exist", folder_path.display());
            exit(1)
        }
    }
    println!("All folders exist");
}

pub trait Skippable {
    /// Get the path for this type
    fn path(&self) -> PathBuf;
    /// Is this skippable ()
    fn skippable(&self) -> bool;
}
