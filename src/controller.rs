use chatgpt::prelude::*;

pub async fn prompt(prompt: String) -> Result<()> {
    // Initial user feedback
    print_initial_info(prompt.clone());

    // API key
    let api_key = "";

    // Creating a client
    let client = ChatGPT::new(api_key)?;

    // Creating a new conversation
    let mut conversation = client.new_conversation();

    let response = conversation
        .send_message(prompt.clone())
        .await?;

    print_answer(response.message().content.clone());

/*     // Sending messages to the conversation
    conversation
        .send_message("Could you describe the Rust programming language in 5 words?")
        .await?;
    let response = conversation
        .send_message("Now could you do the same, but for Kotlin?")
        .await?;
    println!("Response for Kotlin: {}", response.message().content);

    // The history is preserved and is sent to the API each call
    for message in &conversation.history {
        println!("Message in the history: {message:#?}")
    } */

    Ok(())
}

fn print_initial_info(prompt: String) {
    println!("Assemble the robots!");
    println!();
    println!("Question:");
    println!("{}", prompt);
}

fn print_answer(answer: String) {
    println!();
    println!("Answer:");
    println!("{}", answer);
}
