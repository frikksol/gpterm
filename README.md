# gpterm

This is a simple tool for accessing chat-GPT directly from your terminal.

There are two dead simple ways to use it:

### Single prompt mode

Single prompt mode is perfect for quick questions that needs answering:

```bash
gpterm "What is a 25 celsius in kelvin?"
```

### Conversation mode

Conversation mode works like the standard web ui for chat-GPT, where you start a back and forth conversation with the AI:

```bash
gpterm -c
```

## Installation

Currently the only way to install it is by building from source

### Building from source

```bash
cd <wherever>
git clone <repo-name>
cd <repo-name>

# Single prompt mode
cargo run -- "What is 25 Celsius in Kelvin"

# Conversation mode
cargo run -- -c
```
