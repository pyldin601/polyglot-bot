use std::sync::Arc;

use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;
use teloxide::types::InputFile;
use teloxide::utils::command::BotCommands;

use crate::config::Config;
use crate::synth::language::{
    English, Italian, Language, LanguageMeta, Polish, Portuguese, Spanish,
};
use crate::synth::synth::SynthClient;
use crate::text::split_text_into_chunks;

mod config;
mod synth;
mod text;

#[derive(Clone, Default)]
enum State {
    #[default]
    Start,
    ReceiveText(Language),
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
    #[command(description = "read a text in Italian.")]
    Italian,
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
                    let lang = match cmd {
                        Command::Help => {
                            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                                .await?;
                            dialogue.update(State::Start).await?;
                            return Ok(());
                        }
                        Command::Portuguese => Language::Portuguese(Portuguese),
                        Command::Polish => Language::Polish(Polish),
                        Command::English => Language::English(English),
                        Command::Spanish => Language::Spanish(Spanish),
                        Command::Italian => Language::Italian(Italian),
                    };

                    bot.send_message(
                        msg.chat.id,
                        format!("Send me a plain text in {}.", lang.get_name()),
                    )
                    .await?;
                    dialogue.update(State::ReceiveText(lang)).await?;
                    Ok(())
                },
            ))
            .branch(dptree::case![State::Start].endpoint(
                move |bot: Bot, msg: Message, dialogue: MyDialogue| async move {
                    bot.send_message(msg.chat.id, Command::descriptions().to_string())
                        .await?;
                    dialogue.update(State::Start).await?;

                    Ok::<(), anyhow::Error>(())
                },
            ))
            .branch(dptree::case![State::ReceiveText(language)].endpoint(
                |lang: Language,
                 bot: Bot,
                 dialogue: MyDialogue,
                 msg: Message,
                 synth: Arc<SynthClient>| async move {
                    match msg.text() {
                        Some(text) => {
                            let chunks = split_text_into_chunks(text, 5000);

                            let mut audio = vec![];

                            for chunk in chunks {
                                eprintln!("Recording chunk...");
                                let sentence_audio = synth.synth(&chunk, &lang).await?;
                                audio.extend(sentence_audio);
                            }

                            let bytes = bytes::Bytes::from(audio);
                            let file = InputFile::memory(bytes);
                            bot.send_voice(msg.chat.id, file).await?;
                            dialogue.update(State::Start).await?;
                        }
                        None => {
                            bot.send_message(
                                msg.chat.id,
                                format!("Send me a plain text in {}.", lang.get_name()),
                            )
                            .await?;
                        }
                    }

                    Ok(())
                },
            )),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new(), Arc::new(synth)])
    .error_handler(Arc::new(|err| async move {
        eprintln!("Dispatcher error: {:?}", err);
    }))
    .build()
    .dispatch()
    .await;
}
