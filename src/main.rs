use std::sync::Arc;

use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;
use teloxide::types::InputFile;
use teloxide::utils::command::BotCommands;

use crate::config::Config;
use crate::synth::language::{Polish, Portuguese};
use crate::synth::synth::SynthClient;

mod config;
mod synth;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveTextInPortuguese,
    ReceiveTextInPolish,
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "speak a text in portuguese.")]
    Portuguese,
    #[command(description = "speak a text in polish.")]
    Polish,
}

type MyDialogue = Dialogue<State, InMemStorage<State>>;

#[actix_rt::main]
async fn main() {
    let config = Config::from_env();
    let synth = Arc::new(SynthClient::create(&config.ts_api_key));
    let bot = teloxide::Bot::new(config.tg_bot_token);

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
                            bot.send_message(msg.chat.id, "Send me a plain text in portuguese")
                                .await?;
                            dialogue.update(State::ReceiveTextInPortuguese).await?;
                        }
                        Command::Polish => {
                            bot.send_message(msg.chat.id, "Send me a plain text in polish")
                                .await?;
                            dialogue.update(State::ReceiveTextInPolish).await?;
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
            .branch(dptree::case![State::ReceiveTextInPortuguese].endpoint({
                let synth = synth.clone();

                move |bot: Bot, dialogue: MyDialogue, msg: Message| {
                    let synth = synth.clone();

                    async move {
                        match msg.text() {
                            Some(text) => {
                                let audio = synth.synth(text, &Portuguese).await?;
                                let bytes = bytes::Bytes::from(audio);
                                let file = InputFile::memory(bytes)
                                    .file_name(format!("{}.pt.mp3", &text[0..20]));
                                bot.send_audio(msg.chat.id, file).await?;
                                dialogue.update(State::Start).await?;
                            }
                            None => {
                                bot.send_message(msg.chat.id, "Send me a plain text in portuguese")
                                    .await?;
                            }
                        }

                        Ok::<(), anyhow::Error>(())
                    }
                }
            }))
            .branch(dptree::case![State::ReceiveTextInPolish].endpoint({
                let synth = synth.clone();

                move |bot: Bot, dialogue: MyDialogue, msg: Message| {
                    let synth = synth.clone();

                    async move {
                        match msg.text() {
                            Some(text) => {
                                let audio = synth.synth(text, &Polish).await?;
                                let bytes = bytes::Bytes::from(audio);
                                let file = InputFile::memory(bytes)
                                    .file_name(format!("{}.pl.mp3", &text[0..20]));

                                bot.send_audio(msg.chat.id, file).await?;
                                dialogue.update(State::Start).await?;
                            }
                            None => {
                                bot.send_message(msg.chat.id, "Send me a plain text in polish")
                                    .await?;
                            }
                        }

                        Ok::<(), anyhow::Error>(())
                    }
                }
            })),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .build()
    .dispatch()
    .await;
}
