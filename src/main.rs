use clap::Parser;
use dotenv;
use shell::{ Zeus, ZeusSubCommands};
use std::env;
//import modules
mod commands;
mod config;
mod shell;

#[tokio::main]
async fn main() {
    //configure env
    dotenv::dotenv().unwrap();

    //connect to database
    config::database::mongodb().await;
    println!("successfully connected to database");
    //destructure the sub command type from the parse
    let Zeus { action } = Zeus::parse();
    match action {
        ZeusSubCommands::Auth(sub_command) => {
            commands::auth::handle_authorization(sub_command)
        }
        ZeusSubCommands::Chat(sub_command) => commands::chat::handle_chat(sub_command),
        ZeusSubCommands::Config(sub_command) => {
            commands::config::handle_configuration(sub_command)
        }
    }
}
