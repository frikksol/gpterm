use clap::{Parser};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct AppArgs {

    /// The chat-gpt prompt
    #[arg(default_value_t = String::from(""))]
    pub prompt: String,

    /// Start a conversation with chat-GPT
    #[arg(short, long, default_value_t = false)]
    pub conversation: bool,

    /// Delete your config
    #[arg(short, long, default_value_t = false)]
    pub delete_config: bool
}
