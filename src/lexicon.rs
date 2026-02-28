use crate::utils::{to_a, to_i};

#[derive(Debug)]
pub enum ConjugationKind {
    Negation,
    Past,
    Desire,
}

#[derive(Debug)]
pub struct Conjugations {
    pub kinds: Vec<ConjugationKind>,
    pub value: String,
}

impl Conjugations {
    fn consume_last_char(&mut self) {
        if let Some((i, _)) = self.value.char_indices().next_back() {
            self.value = self.value[..i].to_string();
        } else {
            self.value.clear();
        }
    }

    fn conjugate(&mut self, is_godan: &bool, ender: &str, kind: ConjugationKind) {
        match kind {
            ConjugationKind::Negation => {
                // TODO: Validate if kind is even acceptable (can't [negate, polite, negate])

                let conjugation_str = "ない";

                if *is_godan {
                    match self.kinds.last() {
                        // 話ない
                        None => self.value.push(to_a(&ender.chars().last().unwrap())),

                        // 話したくない
                        Some(ConjugationKind::Desire) => {
                            // Consume い
                            self.consume_last_char();
                            self.value.push_str("く");
                        }
                        Some(ConjugationKind::Negation) | Some(ConjugationKind::Past) => {
                            unreachable!()
                        }
                    }
                }
                self.value.push_str(conjugation_str);
            }

            ConjugationKind::Past => {
                let conjugation_str = "た";

                if *is_godan {
                    match self.kinds.last() {
                        // 話した
                        None => self.value.push(to_i(&ender.chars().last().unwrap())),

                        // 話なかった
                        Some(ConjugationKind::Negation) | Some(ConjugationKind::Desire) => {
                            // Consume い
                            self.consume_last_char();
                            self.value.push_str("かっ")
                        }

                        Some(ConjugationKind::Past) => unreachable!(),
                    }
                }
                self.value.push_str(conjugation_str);
            }

            ConjugationKind::Desire => {
                let conjugation_str = "たい";

                if *is_godan {
                    match self.kinds.last() {
                        // 話したい
                        None => self.value.push(to_i(&ender.chars().last().unwrap())),
                        Some(ConjugationKind::Negation)
                        | Some(ConjugationKind::Desire)
                        | Some(ConjugationKind::Past) => unreachable!(),
                    }
                }
                self.value.push_str(conjugation_str);
            }
        }

        self.kinds.push(kind);
    }
}
pub trait Conjugatable {
    fn negate(self) -> Self;
    fn past(self) -> Self;
    fn desire(self) -> Self;
}

#[derive(Debug)]
pub struct Verb {
    pub word: String,
    pub furigana: String,
    pub root: String,
    pub ender: String,
    pub conjugations: Conjugations,
    pub is_godan: bool,
}

impl Conjugatable for Verb {
    fn negate(mut self) -> Self {
        self.conjugations
            .conjugate(&self.is_godan, &self.ender, ConjugationKind::Negation);
        self
    }
    fn past(mut self) -> Self {
        self.conjugations
            .conjugate(&self.is_godan, &self.ender, ConjugationKind::Past);
        self
    }

    fn desire(mut self) -> Self {
        self.conjugations
            .conjugate(&self.is_godan, &self.ender, ConjugationKind::Desire);
        self
    }
}
