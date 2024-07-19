use std::sync::Arc;

use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;
use teloxide::types::InputFile;
use teloxide::utils::command::BotCommands;

use crate::config::Config;
use crate::synth::language::{English, Polish, Portuguese, Spanish};
use crate::synth::synth::SynthClient;

mod config;
mod synth;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveTextInPortuguese,
    ReceiveTextInPolish,
    ReceiveTextInEnglish,
    ReceiveTextInSpanish,
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "read a text in Portuguese.")]
    Portuguese,
    #[command(description = "read a text in Polish.")]
    Polish,
    #[command(description = "read a text in English.")]
    English,
    #[command(description = "read a text in Spanish.")]
    Spanish,
}

type MyDialogue = Dialogue<State, InMemStorage<State>>;

#[actix_rt::main]
async fn main() {
    let config = Config::from_env();
    let synth = SynthClient::create(&config.ts_api_key);
    let bot = Bot::new(config.tg_bot_token);

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(dptree::entry().filter_command::<Command>().endpoint(
                |cmd: Command, msg: Message, bot: Bot, dialogue: MyDialogue| async move {
                    match cmd {
                        Command::Help => {
                            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                                .await?;
                        }
                        Command::Portuguese => {
                            bot.send_message(msg.chat.id, "Send me a plain text in Portuguese.")
                                .await?;
                            dialogue.update(State::ReceiveTextInPortuguese).await?;
                        }
                        Command::Polish => {
                            bot.send_message(msg.chat.id, "Send me a plain text in Polish.")
                                .await?;
                            dialogue.update(State::ReceiveTextInPolish).await?;
                        }
                        Command::English => {
                            bot.send_message(msg.chat.id, "Send me a plain text in English.")
                                .await?;
                            dialogue.update(State::ReceiveTextInEnglish).await?;
                        }
                        Command::Spanish => {
                            bot.send_message(msg.chat.id, "Send me a plain text in Spanish.")
                                .await?;
                            dialogue.update(State::ReceiveTextInSpanish).await?;
                        }
                    }

                    Ok(())
                },
            ))
            .branch(dptree::case![State::Start].endpoint(
                move |bot: Bot, msg: Message| async move {
                    bot.send_message(msg.chat.id, Command::descriptions().to_string())
                        .await?;

                    Ok::<(), anyhow::Error>(())
                },
            ))
            .branch(dptree::case![State::ReceiveTextInPortuguese].endpoint(
                |bot: Bot, dialogue: MyDialogue, msg: Message, synth: Arc<SynthClient>| async move {
                    match msg.text() {
                        Some(text) => {
                            let audio = synth.synth(text, &Portuguese).await?;
                            let bytes = bytes::Bytes::from(audio);
                            let file = InputFile::memory(bytes);
                            bot.send_voice(msg.chat.id, file).await?;
                            dialogue.update(State::Start).await?;
                        }
                        None => {
                            bot.send_message(msg.chat.id, "Send me a plain text in Portuguese.")
                                .await?;
                        }
                    }

                    Ok::<(), anyhow::Error>(())
                },
            ))
            .branch(dptree::case![State::ReceiveTextInPolish].endpoint(
                |bot: Bot, dialogue: MyDialogue, msg: Message, synth: Arc<SynthClient>| async move {
                    match msg.text() {
                        Some(text) => {
                            let audio = synth.synth(text, &Polish).await?;
                            let bytes = bytes::Bytes::from(audio);
                            let file = InputFile::memory(bytes);

                            bot.send_voice(msg.chat.id, file).await?;
                            dialogue.update(State::Start).await?;
                        }
                        None => {
                            bot.send_message(msg.chat.id, "Send me a plain text in Polish.")
                                .await?;
                        }
                    }

                    Ok::<(), anyhow::Error>(())
                },
            ))
            .branch(dptree::case![State::ReceiveTextInEnglish].endpoint(
                |bot: Bot, dialogue: MyDialogue, msg: Message, synth: Arc<SynthClient>| async move {
                    match msg.text() {
                        Some(text) => {
                            let audio = synth.synth(text, &English).await?;
                            let bytes = bytes::Bytes::from(audio);
                            let file = InputFile::memory(bytes);

                            bot.send_voice(msg.chat.id, file).await?;
                            dialogue.update(State::Start).await?;
                        }
                        None => {
                            bot.send_message(msg.chat.id, "Send me a plain text in English.")
                                .await?;
                        }
                    }

                    Ok::<(), anyhow::Error>(())
                },
            ))
            .branch(dptree::case![State::ReceiveTextInSpanish].endpoint(
                |bot: Bot, dialogue: MyDialogue, msg: Message, synth: Arc<SynthClient>| async move {
                    match msg.text() {
                        Some(text) => {
                            let audio = synth.synth(text, &Spanish).await?;
                            let bytes = bytes::Bytes::from(audio);
                            let file = InputFile::memory(bytes);

                            bot.send_voice(msg.chat.id, file).await?;
                            dialogue.update(State::Start).await?;
                        }
                        None => {
                            bot.send_message(msg.chat.id, "Send me a plain text in Spanish.")
                                .await?;
                        }
                    }

                    Ok::<(), anyhow::Error>(())
                },
            )),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new(), Arc::new(synth)])
    .build()
    .dispatch()
    .await;
}
