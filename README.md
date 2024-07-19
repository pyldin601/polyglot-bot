# Polyglot Bot

## Overview

Polyglot Bot is a Text-to-Speech (TTS) Telegram bot built using Rust. This bot converts text messages into speech in
various languages and sends the audio back to the user in the Telegram chat.

## Supported languages

- 🇵🇹 Portuguese
- 🇵🇱 Polish
- 🇬🇧 English
- 🇪🇸 Spanish

## Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/pyldin601/polyglot-bot.git
    cd polyglot-bot
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

3. Configure the bot:
    - Define needed env variables:
      ```env
      export TG_BOT_TOKEN=<your-telegram-bot-token>
      export TS_API_KEY=<your-google-tts-api-key>
      ```

4. Run the bot:
    ```sh
    cargo run --release
    ```

## Usage

- Add the bot to your Telegram group or start a chat with it.
- Use the following commands:
    - `/help` — Display this text.
    - `/portuguese` — Read a text in Portuguese.
    - `/polish` — Read a text in Polish.
    - `/english` — Read a text in English.
    - `/spanish` — Read a text in Spanish.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
