use crate::modules::api_response::{LoginResponse, SeverResponse};
use crate::shell::{AuthSubCommands, User};
use crate::{config, shell::AuthCommands};
use bcrypt::{hash, DEFAULT_COST};
use console::style;
use dialoguer::{theme::ColorfulTheme, Input, Password};
use mongodb::bson::doc;
use rustyline::error::ReadlineError;
use serde_json::json;
use std::fs::File;
use std::io::prelude::*;
// use rustyline::{Editor, Result};

const HELP_INFORMATION: &str = r#"
.clear    Clear the current input
.editor   Enter editor mode
.exit     Exit the REPL
.help     Print this help message
"#;

//chat guide
const CHAT_GUIDE: &str = r#"
\b           begin chat
\e           end conversation
\j <id>      join a chat via id
\i <email>   invite a friend via chat
"#;

//the zeus server url
const ZEUS_SEVER: &str = "http://127.0.0.1:8052/v1/auth/login";


pub async fn handle_authorization(auth_command: AuthCommands) {
    //destructure the sub command
    let AuthCommands { auth_sub_commands } = auth_command;

    //exec the matching sub command
    match auth_sub_commands {
        AuthSubCommands::SignUp => sign_up().await,
        AuthSubCommands::Login => login().await,
    }
}

///accept username and password from cli and log in a user
async fn login() {
    //assign value read from the REPL to username and password fields
    let username: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Username: ")
        .interact_text()
        .unwrap();

    let password: String = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Password: ")
        .interact()
        .unwrap();

    //make request to the zeus server to login the user
    let zeus_server_response = reqwest::Client::new()
        .post(ZEUS_SEVER)
        .header("CONTENT_TYPE", "application/json")
        .header("ACCEPT", "application/json")
        .json(&json!({
        "username":&username,
        "password":&password
        }))
        .send()
        .await
        .unwrap();

    //work with the server response
    match zeus_server_response.status() {
        //if user is not found
        reqwest::StatusCode::NOT_FOUND => {
            println!(
                "{} {} {}",
                style("No account associated with").red().bright(),
                &username,
                style("was found").red().bright()
            )
        }
        //if incorrect password
        reqwest::StatusCode::UNAUTHORIZED => {
            println!(
                "{} {}",
                style("Incorrect Password for").red().bright(),
                &username
            )
        }
        //if the response is ok, get the content
        reqwest::StatusCode::OK => {
            match zeus_server_response
                .json::<SeverResponse<LoginResponse>>()
                .await
            {
                Ok(svr_response) => {
                    //in the password is correct, begin the chat
                    println!(
                        "Successfully logged in as {}\nType \".help\" for more information.\n",
                        &username,
                    );
                    if let Some(data) = svr_response.data {
                        let mut file = File::create("zeus.conf").unwrap();
                        file.write_all(data.token.as_bytes()).unwrap();
                    }

                    //the help information
                    'outer: loop {
                        let mut repl = rustyline::Editor::<()>::new().unwrap();
                        let readline = repl.readline(">> ");
                        //check the user input
                        match readline {
                            Ok(input) => {
                                if input.trim() == ".help" {
                                    println!("{}", &HELP_INFORMATION);
                                } else if input.trim() == ".break" {
                                    break 'outer;
                                } else if input.trim() == ".editor" {
                                    println!("Entered editor mode\n{}", style(CHAT_GUIDE).cyan());
                                     loop {
                                        let mut repl = rustyline::Editor::<()>::new().unwrap();
                                        let prompt = style(">> ").cyan().to_string();
                                        let readline = repl.readline(&prompt);
                                        match readline {
                                            Ok(message) => println!("you entered {}", message),
                                            Err(_) => {
                                                break 'outer;
                                            }
                                        }
                                    }
                                } else if input.trim() == ".exit" {
                                    break 'outer;
                                } else {
                                    println!(
                                        "{}\nType \".help\" for more information",
                                        style("Invalid input.").red()
                                    );
                                }
                            }
                            Err(ReadlineError::Interrupted) => {
                                break 'outer;
                            }
                            Err(ReadlineError::Eof) => {
                                break 'outer;
                            }
                            Err(err) => {
                                println!("An unexpected error occurred{:?}", err);
                                break 'outer;
                            }
                        }
                    }
                }

                Err(_) => println!(
                    "{}",
                    style("An unexpected error occurred, please retry").cyan()
                ),
            }
        }
        //if other errors
        _ => println!(
            "{}",
            style("An unexpected error occurred, please retry").cyan()
        ),
    };
    //generate jwt fo for the user
}

pub async fn sign_up() {
    let username: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Username: ")
        .interact_text()
        .unwrap();

    let password: String = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Password ")
        .interact()
        .unwrap();

    let database = config::database::mongodb().await;
    let collection = database.collection::<User>("user_information");

    //TODO: see if user already exists
    let user_already_exists = collection
        .find_one(doc! {"username:":&username}, None)
        .await
        .unwrap();

    if user_already_exists.is_some() {
        println!(
            "{}",
            style("A user with the provided name already exist").red()
        );
    } else {
        //create a new user
        let hashed_password =
            hash(&password, DEFAULT_COST).expect("expected a password but gon an empty string");
        let user = User {
            username,
            password: hashed_password,
        };

        //save the new user
        collection.insert_one(&user, None).await.unwrap();
        println!("{}", style("User account successfully created").green());
    }
}
