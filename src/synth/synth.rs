use base64::{Engine as _, engine::general_purpose};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::synth::language::{Polish, Portuguese};

const TTS_ENDPOINT: &str = "https://texttospeech.googleapis.com/v1/text:synthesize";

pub(crate) trait SynthParams {
    fn get_voice(&self) -> serde_json::Value;

    fn get_audio_config(&self) -> serde_json::Value;
}

impl SynthParams for Portuguese {
    fn get_voice(&self) -> Value {
        json!({
            "languageCode": "pt-PT",
            "name": "pt-PT-Wavenet-B"
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

pub(crate) struct SynthClient {
    client: reqwest::Client,
    ts_api_key: String,
}

impl SynthClient {
    pub(crate) fn create(ts_api_key: &str) -> SynthClient {
        let client = reqwest::Client::new();

        SynthClient {
            client,
            ts_api_key: String::from(ts_api_key),
        }
    }

    pub(crate) async fn synth(
        &self,
        text: &str,
        language: &(dyn SynthParams + Sync),
    ) -> anyhow::Result<Vec<u8>> {
        let voice = language.get_voice();
        let audio_config = language.get_audio_config();

        #[derive(Deserialize)]
        struct SynthesizeResponse {
            #[serde(rename = "audioContent")]
            audio_content: String,
        }

        let r = self
            .client
            .post(TTS_ENDPOINT)
            .query(&json!({ "key": self.ts_api_key }))
            .json(&json!({
                "input": { "text": text },
                "voice": voice,
                "audioConfig": audio_config
            }))
            .send()
            .await?
            .error_for_status()?
            .json::<SynthesizeResponse>()
            .await?;

        let raw = general_purpose::STANDARD.decode(r.audio_content)?;

        Ok(raw)
    }
}
