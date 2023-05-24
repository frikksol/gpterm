use chatgpt::{err::Error};
use clap::Parser;

mod args;
mod controller;
mod configuration;

use args::RootArgs;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config_result = configuration::get().unwrap(); //TODO avoid unwrap
    let args = RootArgs::parse();

    if args.conversation {
        controller::conversation_prompt(config_result.api_key).await? 
    } else {
        controller::single_prompt(args.prompt, config_result.api_key).await?
    }

    Ok(())
}
