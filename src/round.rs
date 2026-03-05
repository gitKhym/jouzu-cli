use crate::verb::{ConjugationKind, Verb, VerbKind};

#[derive(Debug, Default)]
pub struct Round {
    pub word_to_conjugate: Verb,
    pub conjugations_to_apply: Vec<ConjugationKind>,
}

impl Round {
    pub fn format_conjugations(&self) -> String {
        self.conjugations_to_apply
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn word_ending(&self) -> &str {
        &self.word_to_conjugate.ender
    }

    pub fn word_furigana(&self) -> &str {
        &self.word_to_conjugate.furigana
    }

    pub fn word_base(&self) -> &str {
        &self.word_to_conjugate.base
    }

    pub fn verb_type(&self) -> &str {
        match self.word_to_conjugate.kind {
            VerbKind::Godan => "Godan",
            VerbKind::Ichidan => "Ichidan",
            VerbKind::Irregular => "Irregular",
        }
    }
}
