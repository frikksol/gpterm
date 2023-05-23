use chatgpt::err::Error;
use clap::Parser;

mod args;
mod controller;

use args::RootArgs;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = RootArgs::parse();
    let result = controller::prompt(args.prompt).await;
    if result.is_err() {
        println!("An unhandled error occurred when prompting chat-GTP");
        println!("{:?}", result);
    }
    result
}
