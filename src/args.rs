use clap::{Parser};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct AppArgs {

    /// The chat-gpt prompt
    pub prompt: String,

    /// Enable to start a conversation instead of a single prompt
    #[arg(short, long, default_value_t = false)]
    pub conversation: bool
}
