use clap::{Args, Parser, SubCommand};

//define the cli commands parser
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Zeus {
    #[clap(subcommand)]
    pub command: ZeusSubCommands,
}

/**
 * the cli will take 3 (or more) sub commands
 * - auth - to authorize user login and sigh up
 * - config - to configure a user interface
 * - chat - to create a chat id or connect to one
 */
#[derive(clap::Subcommand, Debug)]
pub enum ZeusSubCommands {
    ///authorize a user
    Auth(AuthCommands),
    //configure the color, typeface, email back up e.t.c
    Config(ConfigCommands),
    //connect to a message Id
    Chat(ConnectCommands),
}

/*
* the auth commands is supposed to be an enum by default
* instead it will be a struct that takes an enum (AuthSubCommands)
*/
#[derive(Args, Debug)]
pub struct AuthCommands {
    auth_sub_commands: AuthSubCommands,
}

/** the authorization sub commands  */
#[derive(clap::Subcommand, Debug)]
pub enum AuthSubCommands {
    ///takes in username, and password, creates a new user
    SignUpCommand(AuthSignUp),
    /// takes in username and password, logs a user in to the zeus organization
    LoginCommand(AuthLogin),
}

//implementation of the sigh up sub commands
#[derive(Args, Debug)]
pub struct AuthSignUp {
     /// the username
    #[clap(short, long, value_parser, forbid_empty_values = true)]
    pub username: String,
    /// the user password
    #[clap(short, long, value_parser, forbid_empty_values = true)]
    pub password: String,
}

//implementation of the login commands
#[derive(Args, Debug)]
struct AuthLogin {
    /// the username
    #[clap(short, long, value_parser, forbid_empty_values = true)]
    pub username: String,
    /// the user password
    #[clap(short, long, value_parser, forbid_empty_values = true)]
    pub password: String,
}



#[derive(Args, Debug)]
pub struct ConnectCommands {}

//config commands
#[derive(Args, Debug)]
pub struct ConfigCommands {}
