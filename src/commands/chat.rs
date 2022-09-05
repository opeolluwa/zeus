use crate::shell::{ChatCommands, ChatSubCommands};

fn _chat() {
    todo!();
}

pub fn handle_chat(command: ChatCommands) {
    //destructure the command type from the sub_command argument
    let ChatCommands { chat_sub_commands } = command;
    match chat_sub_commands {
        ChatSubCommands::BeginChat => print!("begin"),
        ChatSubCommands::SendChatInvitation => print!("sent invite"),
        ChatSubCommands::JoinChat => print!("join cht"),
    }
}
