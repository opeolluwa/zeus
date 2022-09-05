use crate::shell::{AuthSubCommands, User};
use crate::{config, shell::AuthCommands};
use dialoguer::{Input, Password};
use mongodb::bson::doc;

pub async fn handle_authorization(auth_command: AuthCommands) {
    //destructure the sub command
    let AuthCommands { auth_sub_commands } = auth_command;

    //exec the matching sub command
    match auth_sub_commands {
        AuthSubCommands::SignUp => sign_up(),
        AuthSubCommands::Login => login().await,
    }
}

///accept username and password from cli and log in a user
async fn login() {
    //assign value
    let username: String = Input::new()
        .with_prompt("username? ")
        .interact_text()
        .unwrap();

    let password: String = Password::new()
        .with_prompt("password? ")
        .interact()
        .unwrap();
    // println!(" username {}, password {}", username, password);

    //connect to database, check if user details is valid
    let database = config::database::mongodb().await;
    let collection = database.collection::<User>("user_information");

    let result = collection
        .find_one(doc! { "username": &username }, None)
        .await
        .unwrap();

    //try to destructure the found object
     if let Some(User {
        username, password, ..
    }) = result
    {
        // (username, password)
    println!(" username {}, password {}", username, password);

    } else {
        //if no user was found return 404 error
        println!("User with provided credentials not found");
        return;
    };

    //generate jwt fo for the user 
}

pub fn sign_up() {}
