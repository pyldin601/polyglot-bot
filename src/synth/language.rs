use serde_json::{json, Value};

use crate::synth::synth::SynthParams;

pub(crate) trait LanguageMeta {
    fn get_name(&self) -> &'static str;
}

#[derive(Clone)]
pub(crate) struct Polish;

impl LanguageMeta for Polish {
    fn get_name(&self) -> &'static str {
        "Polish"
    }
}

impl SynthParams for Polish {
    fn get_voice(&self) -> Value {
        json!({
            "languageCode": "pl-PL",
            "name": "pl-PL-Wavenet-B"
        })
    }

    fn get_audio_config(&self) -> Value {
        json!({
          "audioEncoding": "OGG_OPUS",
          "speakingRate": 1
        })
    }
}

#[derive(Clone)]
pub(crate) struct Portuguese;

impl LanguageMeta for Portuguese {
    fn get_name(&self) -> &'static str {
        "Portuguese"
    }
}

impl SynthParams for Portuguese {
    fn get_voice(&self) -> Value {
        json!({
            "languageCode": "pt-PT",
            "name": "pt-PT-Wavenet-C"
        })
    }

    fn get_audio_config(&self) -> Value {
        json!({
          "audioEncoding": "OGG_OPUS",
          "speakingRate": 0.8
        })
    }
}

#[derive(Clone)]
pub(crate) struct English;

impl LanguageMeta for English {
    fn get_name(&self) -> &'static str {
        "English"
    }
}

impl SynthParams for English {
    fn get_voice(&self) -> Value {
        json!({
            "languageCode": "en-GB",
            "name": "en-GB-Neural2-B"
        })
    }

    fn get_audio_config(&self) -> Value {
        json!({
          "audioEncoding": "OGG_OPUS",
          "speakingRate": 1
        })
    }
}

#[derive(Clone)]
pub(crate) struct Spanish;

impl LanguageMeta for Spanish {
    fn get_name(&self) -> &'static str {
        "Spanish"
    }
}

impl SynthParams for Spanish {
    fn get_voice(&self) -> Value {
        json!({
            "languageCode": "es-ES",
            "name": "es-ES-Neural2-F"
        })
    }

    fn get_audio_config(&self) -> Value {
        json!({
          "audioEncoding": "OGG_OPUS",
          "speakingRate": 1
        })
    }
}

#[derive(Clone)]
pub(crate) struct Italian;

impl LanguageMeta for Italian {
    fn get_name(&self) -> &'static str {
        "Italian"
    }
}

impl SynthParams for Italian {
    fn get_voice(&self) -> Value {
        json!({
            "languageCode": "it-IT",
            "name": "it-IT-Chirp3-HD-Zephyr"
        })
    }

    fn get_audio_config(&self) -> Value {
        json!({
          "audioEncoding": "OGG_OPUS",
          "speakingRate": 1
        })
    }
}

#[derive(Clone)]
pub(crate) enum Language {
    Polish(Polish),
    Portuguese(Portuguese),
    English(English),
    Spanish(Spanish),
    Italian(Italian),
}

impl LanguageMeta for Language {
    fn get_name(&self) -> &'static str {
        match self {
            Language::Polish(lang) => lang.get_name(),
            Language::Portuguese(lang) => lang.get_name(),
            Language::English(lang) => lang.get_name(),
            Language::Spanish(lang) => lang.get_name(),
            Language::Italian(lang) => lang.get_name(),
        }
    }
}

impl SynthParams for Language {
    fn get_voice(&self) -> Value {
        match self {
            Language::Polish(params) => params.get_voice(),
            Language::Portuguese(params) => params.get_voice(),
            Language::English(params) => params.get_voice(),
            Language::Spanish(params) => params.get_voice(),
            Language::Italian(params) => params.get_voice(),
        }
    }

    fn get_audio_config(&self) -> Value {
        match self {
            Language::Polish(params) => params.get_audio_config(),
            Language::Portuguese(params) => params.get_audio_config(),
            Language::English(params) => params.get_audio_config(),
            Language::Spanish(params) => params.get_audio_config(),
            Language::Italian(params) => params.get_audio_config(),
        }
    }
}
