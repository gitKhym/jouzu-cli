use crate::verb::{ConjugationKind, Verb};

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
}
