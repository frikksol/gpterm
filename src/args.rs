use clap::{Parser};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct RootArgs {

    /// The chat-gpt prompt
    pub prompt: String,
}
