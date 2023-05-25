# gpterm

This is a simple tool for accessing chat-GPT directly from your terminal.

The first time you run the application it will guide you on how to add an API key from chat-GPT.

There are two dead simple ways to use it:

### Single prompt mode

Single prompt mode is perfect for quick questions that needs answering:

```bash
gpterm "What is a 25 celsius in kelvin?"
```

![single-prompt-mode](https://github.com/frikksol/gpterm/assets/13680486/57bb62a9-6f70-4732-9aa3-b866948ff789)

### Conversation mode

Conversation mode works like the standard web ui for chat-GPT, where you start a back and forth conversation with the AI:

```bash
gpterm -c
```

![conversation-mode](https://github.com/frikksol/gpterm/assets/13680486/d080a8eb-3494-437d-be83-0a100e59069b)

## Prerequisites

- A chat-GPT user and API key. After creating a user, go to https://platform.openai.com/account/api-keys to create one

## Installation

Currently the only way to install it is by installing with Cargo or building from source. I am hoping to soon add it to package managers.

### From Cargo

```bash
cargo install gpterm

# Single prompt mode
gpterm "What is a 25 celsius in kelvin?"

# Conversation mode
gpterm -c
```

### Building from source

```bash
cd <wherever>
git clone git@github.com:frikksol/gpterm.git
cd gpterm

# Single prompt mode
cargo run -- "What is 25 Celsius in Kelvin"

# Conversation mode
cargo run -- -c
```
