use clap::Parser;
use facebook_data_parser::activity::messages::{list_files, reorg_images, reorg_videos};
// use enum_iterator::all;
// use facebook_data_parser::activity::ActivityTypes;
use facebook_data_parser::{
    folder_checks, ActivityActivity, ActivityMessagesSubCommand, CliCommand, CliCommands,
};

fn main() {
    let cliopts = CliCommand::parse();

    folder_checks();

    // eprintln!("CliOpts: {:?}", cliopts);

    match cliopts.command {
        CliCommands::Activity { command } => match command {
            ActivityActivity::Messages(msg) => {
                match msg.command {
                    ActivityMessagesSubCommand::ReorgImages => {
                        reorg_images(msg).expect("Failed to reorg messages");
                    }
                    ActivityMessagesSubCommand::ReorgVideos => {
                        reorg_videos(msg).expect("Failed to reorg videos");
                    }
                    ActivityMessagesSubCommand::ListFiles => {
                        list_files(msg).expect("Failed to list files")
                    }
                }
                // reorg_images(msg).expect("Failed to reorg messages");
            }
        },
    }
}
