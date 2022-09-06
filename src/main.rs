use clap::Parser;
use shell::{Zeus, ZeusSubCommands};
//import modules
mod commands;
mod config;
mod modules;
mod shell;

#[tokio::main]
async fn main() {
    //configure env
    dotenv::dotenv().unwrap();

    //destructure the sub command type from the parse
    let Zeus { command } = Zeus::parse();
    match command {
        ZeusSubCommands::Auth(sub_cmd) => commands::auth::handle_authorization(sub_cmd).await,
        ZeusSubCommands::Chat(sub_cmd) => commands::chat::handle_chat(sub_cmd),
        ZeusSubCommands::Config(sub_cmd) => commands::config::handle_configuration(sub_cmd),
    }
}
