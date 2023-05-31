use std::io;

use chatgpt::prelude::*;
use loading::Loading;
use termimad::MadSkin;

pub async fn single_prompt(prompt: String, api_key: String) -> Result<()> {
    print_single_start(prompt.clone());

    let client = ChatGPT::new(api_key)?;
    let mut conversation = client.new_conversation();

    let loading = Loading::default();
    loading.text("Waiting for the AI to do AI things");

    let response = conversation
        .send_message(prompt.clone())
        .await?;

    loading.success("Answer:");
    loading.end();

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
        io::stdin().read_line(&mut question_buffer)?;
        let question = question_buffer.trim_end_matches("\n").to_string();
        match question.as_str() {
            "quit" | "q" => {
                print_conversation_exit();
                finished = true;
            }
            _ => {
                let loading = Loading::default();
                loading.text("Waiting for the AI to do AI things");

                let response = conversation
                    .send_message(question)
                    .await?;

                loading.success("Answer:");
                loading.end();
                print_answer(response.message().content.clone());
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
    let skin = MadSkin::default();
    skin.print_text(answer.as_str());
}
