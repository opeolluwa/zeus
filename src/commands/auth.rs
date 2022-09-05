use crate::shell::{AuthSubCommands, User};
use crate::{config, shell::AuthCommands};
use bcrypt::{hash, verify, DEFAULT_COST};
use console::style;
use dialoguer::{theme::ColorfulTheme, Input, Password};
use mongodb::bson::doc;

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
            Ok(_) => println!(
                "
                
                {} {}",
                style("successfully logged in as").green(),
                &username
            ),
            Err(_) => println!("{} {}", style("incorrect password for ").red(), &username),
        }
        println!(
            " username {}, password {:?}, hash {}",
            username, password, &hashed_password
        );
    } else {
        //if no user was found return 404 error
        println!(
            "{}",
            style("User with provided credentials not found").red()
        );
        return;
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

    //see if user already exists
    let user_already_exists = collection
        .find_one(doc! {"username:":&username}, None)
        .await
        .unwrap();

    if let Some(_) = user_already_exists {
        println!(
            "{}",
            style("A user with the provided name already exist").red()
        );
        return;
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
