use clap::Parser;
use facebook_data_parser::activity::messages::{
    list_files, reorg_images, reorg_videos, search_messages,
};
// use enum_iterator::all;
// use facebook_data_parser::activity::ActivityTypes;
use facebook_data_parser::{
    folder_checks, ActivityActivity, ActivityMessagesSubCommand, CliCommand, CliCommands,
};

fn main() {
    let cliopts = CliCommand::parse();

    folder_checks();

    match cliopts.command {
        CliCommands::Activity { command } => match command {
            ActivityActivity::Messages(msg) => match msg.command {
                ActivityMessagesSubCommand::ReorgImages => {
                    reorg_images(msg).expect("Failed to reorg messages");
                }
                ActivityMessagesSubCommand::ReorgVideos => {
                    reorg_videos(msg).expect("Failed to reorg videos");
                }
                ActivityMessagesSubCommand::ListFiles => {
                    list_files(msg).expect("Failed to list files")
                }
                ActivityMessagesSubCommand::SearchMessages { path } => {
                    search_messages(path).expect("Failed to search messages")
                }
            },
        },
        CliCommands::Search {
            earliest,
            latest,
            types,
            show_paths,
            has_attachments,
        } => {
            let search_options = facebook_data_parser::search::SearchOptions::new(
                earliest,
                latest,
                types,
                show_paths,
                has_attachments,
            )
            .expect("Failed to parse search options");
            let results = facebook_data_parser::search::search(search_options)
                .expect("Failed to perform search");

            for result in results {
                println!("{}", result);
            }
        }
    }
}
