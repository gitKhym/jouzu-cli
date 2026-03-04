use std::fmt;

use serde::Deserialize;

use crate::utils::{is_voiced, last, to_a, to_i};

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Copy, Clone)]
pub enum ConjugationKind {
    Polite = 1,
    Passive = 2,
    Continuous = 3,
    Desire = 4,
    Negation = 5,
    Past = 6,
    Te = 7,
}

impl fmt::Display for ConjugationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConjugationKind::Polite => write!(f, "Polite"),
            ConjugationKind::Passive => write!(f, "Passive"),
            ConjugationKind::Continuous => write!(f, "Continuous"),
            ConjugationKind::Desire => write!(f, "たい form"),
            ConjugationKind::Negation => write!(f, "Negative"),
            ConjugationKind::Past => write!(f, "Past"),
            ConjugationKind::Te => write!(f, "て form"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Deserialize, Clone)]
// example verb: 話す
pub struct Verb {
    pub furigana: String, // はな
    pub base: String,     // 話
    pub ender: String,    // す
    pub is_godan: bool,   // true
}

#[derive(Debug)]
pub enum ConjugationError {
    InvalidCombinations(String),
}

impl Verb {
    pub fn godan(base: &str, ender: &str, furigana: &str) -> Self {
        Verb {
            furigana: furigana.to_string(),
            base: base.to_string(),
            ender: ender.to_string(),
            is_godan: true,
        }
    }

    // pub fn ichidan(base: &str, furigana: &str) -> Self {
    //     Verb {
    //         furigana: furigana.to_string(),
    //         base_form: base.to_string(),
    //         is_godan: false,
    //     }
    // }

    pub fn conjugate(
        &mut self,
        transformations: &mut Vec<ConjugationKind>,
    ) -> Result<(), ConjugationError> {
        transformations.sort();

        let mut previous_conjugation = None;
        for transformation in transformations {
            self.apply_conjunction(&transformation, previous_conjugation)
                .unwrap();
            previous_conjugation = Some(transformation);
        }

        Ok(())
    }

    pub fn get_word(&self) -> String {
        format!("{}{}", self.base, self.ender)
    }

    pub fn get_hiragana(&self) -> String {
        format!("{}{}", self.furigana, self.ender)
    }

    fn apply_conjunction(
        &mut self,
        kind: &ConjugationKind,
        previous: Option<&ConjugationKind>,
    ) -> Result<(), ConjugationError> {
        match kind {
            ConjugationKind::Negation => {
                self.negative(previous);
                Ok(())
            }
            ConjugationKind::Passive => {
                self.passive(previous);
                Ok(())
            }
            ConjugationKind::Past => {
                self.past(previous);
                Ok(())
            }
            ConjugationKind::Desire => {
                self.desire(previous);
                Ok(())
            }
            ConjugationKind::Te => {
                self.te_form(previous);
                Ok(())
            }
            ConjugationKind::Continuous => {
                self.continous(previous);
                Ok(())
            }
            ConjugationKind::Polite => {
                todo!()
            }
        }
    }
    fn passive(&mut self, previous: Option<&ConjugationKind>) {
        let conjugation_str = "れる";
        match previous {
            None => {
                let last = last(&self.ender);
                self.ender.pop();
                self.ender.push(to_a(&last));
            }
            _ => panic!("Incorrect grammar, wrong conjugation precedence"),
        }

        self.ender.push_str(conjugation_str);
    }

    fn negative(&mut self, previous: Option<&ConjugationKind>) {
        let conjugation_str = "ない";

        match previous {
            None => {
                if self.is_godan {
                    let last = last(&self.ender);
                    self.ender.pop();
                    self.ender.push(to_a(&last));
                } else {
                    self.ender.pop();
                }
            }
            Some(ConjugationKind::Desire) => {
                // Consume い
                self.ender.pop();
                self.ender.push_str("く")
            }
            Some(ConjugationKind::Passive) | Some(ConjugationKind::Continuous) => {
                // Consume る
                self.ender.pop();
            }
            _ => panic!("Incorrect grammar, wrong conjugation precedence"),
        }

        self.ender.push_str(conjugation_str);
    }

    fn past(&mut self, previous: Option<&ConjugationKind>) {
        let conjugation_str = if is_voiced(&self.ender) { "だ" } else { "た" };

        match previous {
            None => {
                if self.is_godan {
                    let last = last(&self.ender);
                    self.ender.pop();

                    match last {
                        'る' | 'う' | 'つ' => self.ender.push('っ'),
                        'ぬ' | 'ぶ' | 'む' => self.ender.push('ん'),
                        'く' | 'ぐ' => self.ender.push('い'),
                        'す' => self.ender.push('し'),
                        _ => panic!("Invalid verb ending: {}", last),
                    }
                } else {
                    self.ender.pop();
                }
            }
            Some(ConjugationKind::Negation) | Some(ConjugationKind::Desire) => {
                // Consume い
                self.ender.pop();
                self.ender.push_str("かっ")
            }
            Some(ConjugationKind::Passive) | Some(ConjugationKind::Continuous) => {
                // Consume る
                self.ender.pop();
            }
            _ => panic!(
                "Incorrect grammar, wrong conjugation precedence, previous: {:?}",
                previous
            ),
        }

        self.ender.push_str(conjugation_str);
    }

    fn desire(&mut self, previous: Option<&ConjugationKind>) {
        let conjugation_str = "たい";

        if self.is_godan {
            match previous {
                None => {
                    let last = last(&self.ender);
                    self.ender.pop();
                    self.ender.push(to_i(&last));
                }
                Some(ConjugationKind::Negation) => {
                    // Consume い
                    self.ender.pop();
                    self.ender.push_str("かっ")
                }
                Some(ConjugationKind::Passive) | Some(ConjugationKind::Continuous) => {
                    // Consume る
                    self.ender.pop();
                }
                _ => panic!(
                    "Incorrect grammar, wrong conjugation precedence, previous: {:?}",
                    previous
                ),
            }
        } else {
            self.ender.pop();
        }

        self.ender.push_str(conjugation_str);
    }

    fn continous(&mut self, previous: Option<&ConjugationKind>) {
        let mut conjugation_str = "いる";

        match previous {
            None => {
                if self.is_godan {
                    conjugation_str = "いる";

                    let last = last(&self.ender);
                    self.ender.pop();

                    match last {
                        'ぬ' | 'ぶ' | 'む' => self.ender.push_str("んで"),
                        'ぐ' => self.ender.push_str("いで"),
                        'る' | 'う' | 'つ' => self.ender.push_str("って"),
                        'く' => self.ender.push_str("いて"),
                        'す' => self.ender.push_str("して"),
                        _ => panic!("Invalid verb ending: {}", last),
                    }
                } else {
                    self.ender.pop();
                }
            }

            Some(ConjugationKind::Negation) | Some(ConjugationKind::Desire) => {
                // Consume い
                self.ender.pop();
                self.ender.push_str("く")
            }
            Some(ConjugationKind::Passive) => {
                // Consume る
                self.ender.pop();
            }

            _ => panic!(
                "Incorrect grammar, wrong conjugation precedence, previous: {:?}",
                previous
            ),
        }

        self.ender.push_str(conjugation_str);
    }

    fn te_form(&mut self, previous: Option<&ConjugationKind>) {
        let conjugation_str = if is_voiced(&self.ender) { "で" } else { "て" };

        match previous {
            None => {
                if self.is_godan {
                    let last = last(&self.ender);
                    self.ender.pop();

                    match last {
                        'る' | 'う' | 'つ' => self.ender.push('っ'),
                        'ぬ' | 'ぶ' | 'む' => self.ender.push('ん'),
                        'く' | 'ぐ' => self.ender.push('い'),
                        'す' => self.ender.push('し'),
                        _ => panic!("Invalid verb ending: {}", last),
                    }
                } else {
                    self.ender.pop();
                }
            }

            Some(ConjugationKind::Negation) | Some(ConjugationKind::Desire) => {
                // Consume い
                self.ender.pop();
                self.ender.push_str("く")
            }
            Some(ConjugationKind::Passive) => {
                // Consume る
                self.ender.pop();
            }

            // Some(ConjugationKind::Past | ConjugationKind::Continuous) => {
            //     let msg = match previous.unwrap() {
            //         ConjugationKind::Past => "Te form cannot be combined with Past form",
            //         ConjugationKind::Continuous => {
            //             "Te form cannot be combined with Continuous form"
            //         }
            //         _ => unreachable!(),
            //     };
            //     return Err(ConjugationError::InvalidCombinations(msg.to_string()));
            // }
            _ => panic!(
                "Incorrect grammar, wrong conjugation precedence, previous: {:?}",
                previous
            ),
        }

        self.ender.push_str(conjugation_str);
    }
}
