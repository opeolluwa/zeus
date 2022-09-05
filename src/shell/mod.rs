use clap::{Args, Parser, Subcommand};
/*
the cli will take 3 (or more) sub commands
 - auth - to authorize user login and sigh up
 - config - to configure a user interface
 - chat - to create a chat id or connect to one
*/
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Zeus {
    #[clap(subcommand)]
    pub action: ZeusSubCommands,
}

#[derive(clap::Subcommand, Debug)]
pub enum ZeusSubCommands {
    ///create account or login
    Auth(AuthCommands),
    ///configure the color, typeface, email back up e.t.c
    Config(ConfigCommands),
    ///begin a conversation, send invites or join a chat
    Chat(ChatCommands),
}

/*
* the auth commands is supposed to be an enum by default
* instead it will be a struct that takes an enum (AuthSubCommands)
*/
#[derive(Args, Debug)]
pub struct AuthCommands {
    #[clap(subcommand)]
    pub auth_sub_commands: AuthSubCommands,
}

/** the authorization sub commands  */
#[derive(clap::Subcommand, Debug)]
pub enum AuthSubCommands {
    ///takes in username, and password, creates a new user
    Register(User),
    /// takes in username and password, logs a user in to the zeus organization
    Login(User),
}

//implementation of the sigh up sub commands
#[derive(Args, Debug)]
pub struct User {
    /// the username
    #[clap(short, long, value_parser,/*  forbid_empty_values = true */)]
    pub username: String,
    /// the user password
    #[clap(short, long, value_parser,/*  forbid_empty_values = true */)]
    pub password: String,
}



#[derive(Args, Debug)]
pub struct ChatCommands {
    #[clap(subcommand)]
    chat_sub_commands: ChatSubCommands,
}

//chat sub commands
#[derive(Subcommand, Debug)]
pub enum ChatSubCommands {
    ///generate a chat id for others to join
    BeginChatCommand(BeginChat),
    ///join a chat via id
    JoinChatCommand(JoinChat),
    /// send chat invitation via email
    SendChatInvitation(SendChatInvitation),
}

// begin chat
#[derive(Args, Debug)]
pub struct BeginChat {
    pub chat_id: String,
}

// begin chat
#[derive(Args, Debug)]
pub struct JoinChat {
    //TODO: #[derive(serde(rename="chatId"))]
    /// the chat id
    pub chat_id: String,
}

///share chat invitaion via email
#[derive(Args, Debug)]
pub struct SendChatInvitation {
    /// the chat id
    pub chat_id: String,
}
//config commands
#[derive(Args, Debug)]
pub struct ConfigCommands {}
