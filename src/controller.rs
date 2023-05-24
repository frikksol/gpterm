use std::io;

use chatgpt::prelude::*;

pub async fn single_prompt(prompt: String, api_key: String) -> Result<()> {
    print_single_start(prompt.clone());

    let client = ChatGPT::new(api_key)?;
    let mut conversation = client.new_conversation();

    let response = conversation
        .send_message(prompt.clone())
        .await?;

    print_answer(response.message().content.clone());

    Ok(())
}

pub async fn conversation_prompt(api_key: String) -> Result<()> {
    print_conversation_start();

    let mut finished = false;
    let client = ChatGPT::new(api_key)?;
    let mut conversation = client.new_conversation();

    while !finished {
        print_conversation_question();

        let mut question_buffer = String::new();
        let read_result = io::stdin().read_line(&mut question_buffer);
        if read_result.is_ok() { // TODO Error handling
            let question = question_buffer.trim_end_matches("\n").to_string();
            match question.as_str() {
                "quit" | "q" => {
                    print_conversation_exit();
                    finished = true;
                }
                _ => {
                    let response = conversation
                        .send_message(question)
                        .await?;
                    print_answer(response.message().content.clone());
                }
            }
        }
    }
    Ok(())
}

fn print_single_start(prompt: String) {
    println!();
    println!("🧐 Question:");
    println!("{}", prompt);
}

fn print_conversation_start() {
    println!();
    println!("✨ Starting a conversation with chat-GPT")
}

fn print_conversation_question() {
    println!();
    println!("🧐 Enter your question below. (type \"quit\" or \"q\" to quit)");
}

fn print_conversation_exit() {
    println!();
    println!("👋 Ok, bye!");
}

fn print_answer(answer: String) {
    println!();
    println!("📢 Answer:");
    println!("{}", answer);
}
