use clap::Parser;
use dotenv;
use shell::{Zeus, ZeusSubCommands};
use std::env;
//import modules
mod commands;
mod config;
mod shell;

#[tokio::main]
async fn main() {
    //configure env
    dotenv::dotenv();

    //connect to database
    config::database::mongodb().await;
    println!("successfully connected to database");
    //destructure the sub command type from the parse
    let Zeus { sub_command } = Zeus::parse();

    match sub_command {
        ZeusSubCommands::Auth(auth) => {
            println!("{:#?}", auth);
        }

        ZeusSubCommands::Chat(chat) => {
            //check if user is authorized before exec controllers
            println!("{:#?}", chat);
        }

        ZeusSubCommands::Config(config) => {
            //check if user is authorized before running controllers
            println!("{:#?}", config);
        }
    }
}
