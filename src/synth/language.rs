use std::ops::Deref;

use crate::synth::synth::SynthParams;

pub(crate) struct Polish;

pub(crate) struct Portuguese;

pub(crate) enum SynthLanguage {
    Polish(Polish),
    Portuguese(Portuguese),
}

impl Deref for SynthLanguage {
    type Target = dyn SynthParams;

    fn deref(&self) -> &Self::Target {
        match self {
            SynthLanguage::Polish(pl) => pl,
            SynthLanguage::Portuguese(pt) => pt,
        }
    }
}
