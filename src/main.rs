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
    let result = controller::prompt(args.prompt, config_result.api_key).await;
    if result.is_err() {
        println!("An unhandled error occurred when prompting chat-GTP");
        println!("{:?}", result);
    }
    result
}
