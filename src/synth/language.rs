use serde_json::{json, Value};

use crate::synth::synth::SynthParams;

pub(crate) struct Polish;

pub(crate) struct Portuguese;

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
