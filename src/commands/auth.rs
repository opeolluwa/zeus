use crate::shell::{User, AuthSubCommands};
use crate::{config::database::mongodb, shell::AuthCommands};
use dialoguer::{Input, Password};
use mongodb::bson::doc;

pub fn handle_authorization(auth_command: AuthCommands) {
    //destructure the sub command
    let AuthCommands { auth_sub_commands } = auth_command;

    //exec the matching sub command
 match auth_sub_commands {
        AuthSubCommands::Register(user) => println!("{:#?}", user),
        AuthSubCommands::Login(user) => login(user)
    }
}


///accept username and password from cli and log in a user
  fn login(user:User) {
    //see if user already exists
    // let AuthLogin { username, password } = credentials;
    let username: String = Input::new()
        .with_prompt("username? ")
        .interact_text()
        .unwrap();

    let password = Password::new()
        .with_prompt("password? ")
        .interact()
        .unwrap();

    println!(" username {} password {}", username, password);

    //see if user already exists
}

pub fn sign_up() {}
