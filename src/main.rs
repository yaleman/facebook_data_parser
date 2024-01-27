use clap::Parser;
use facebook_data_parser::activity::messages::reorg_images;
// use enum_iterator::all;
// use facebook_data_parser::activity::ActivityTypes;
use facebook_data_parser::{folder_checks, ActivityActivity, CliCommand, CliCommands};

fn main() {
    println!("Starting");

    let cliopts = CliCommand::parse();

    folder_checks();

    eprintln!("CliOpts: {:?}", cliopts);

    match cliopts.command {
        CliCommands::Activity { command } => match command {
            ActivityActivity::Messages(msg) => {
                reorg_images(msg).expect("Failed to reorg messages");
            }
        },
    }
}
