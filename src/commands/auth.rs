use crate::shell::{AuthSubCommands, User};
use crate::{config, shell::AuthCommands};
use bcrypt::{hash, verify, DEFAULT_COST};
use console::style;
use dialoguer::{theme::ColorfulTheme, Input, Password};
use mongodb::bson::doc;
use rustyline::error::ReadlineError;
// use rustyline::{Editor, Result};

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
    //assign value
    let username: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Username: ")
        .interact_text()
        .unwrap();

    //connect to database, check if user details is valid
    let database = config::database::mongodb().await;
    let collection = database.collection::<User>("user_information");

    let result = collection
        .find_one(doc! { "username": &username }, None)
        .await
        .unwrap();

    //try to destructure the found object, then ask for password
    if let Some(User {
        username,
        password: hashed_password,
        ..
    }) = result
    {
        let password: String = Password::with_theme(&ColorfulTheme::default())
            .with_prompt("Password: ")
            .interact()
            .unwrap();

        //check for correctness of the pasword
        let is_correct_password = verify(&password, &hashed_password);
        match is_correct_password {
            Ok(correct_password) => {
                //destruct password
                if !correct_password {
                    println!("{} {}", style("Incorrect Password for ").red(), &username);
                    return;
                }
            }
            //inform the user of the error
            Err(_) => {
                println!("{} {}", style("Error authorizing",).red(), &username);
                return;
            }
        }

        //in the password is correct, begin the chat
        println!(
            "Successfully logged in as {}\nType \".help\" for more information.\n",
            &username,
        );

        //the help information
        let help_information = r#"
.clear    Clear the current input
.editor   Enter editor mode
.exit     Exit the REPL
.help     Print this help message
        "#;

        //chat guide
        let chat_guide = r#"
\b           begin chat
\i <email>   invite a friend via chat
\e           end conversation
\j <id>      join a chat via id
        "#;
        // println!("{}", &help_information);
        // define the repl of the chat
        loop {
            let mut repl = rustyline::Editor::<()>::new().unwrap();
            let readline = repl.readline(">> ");
            //check the user input
            match readline {
                Ok(input) => {
                    if input.trim() == ".help" {
                        println!("{}", &help_information);
                    } else if input.trim() == ".break" {
                        break;
                    } else if input.trim() == ".editor" {
                        loop {
                            println!("Entered editor mode\n{}", style(chat_guide).cyan());
                            let mut repl = rustyline::Editor::<()>::new().unwrap();
                            let prompt = style(">> ").cyan().to_string();
                            let readline = repl.readline(&prompt);
                            match readline {
                                Ok(message) => println!("you entered {}", message),
                                Err(_) => {
                                    println!("an error occurred");
                                    break ;
                                }
                            }
                        }
                    } else if input.trim() == ".exit" {
                        break;
                    } else {
                        println!(
                            "{}\nType \".help\" for more information",
                            style("Invalid input.").red()
                        );
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("An unexpected error occurred{:?}", err);
                    break;
                }
            }
        }
    } else {
        //if no user was found return 404 error
        println!(
            "{}",
            style("User with provided credentials not found").red()
        );
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
