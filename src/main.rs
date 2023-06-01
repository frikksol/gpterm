use chatgpt::{err::Error};
use clap::Parser;

mod args;
mod controller;
mod configuration;

use args::AppArgs;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = AppArgs::parse();
    let config_result = configuration::get().unwrap();

    if args.delete_config{
        configuration::delete().unwrap();
    } else if args.conversation {
        controller::conversation_prompt(config_result.api_key).await? 
    } else {
        controller::single_prompt(args.prompt, config_result.api_key).await?
    }
    Ok(())
}
