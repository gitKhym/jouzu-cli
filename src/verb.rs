use std::fmt;

use serde::Deserialize;

use crate::utils::{is_voiced, last, to_a, to_i};

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Copy, Clone)]
pub enum ConjugationKind {
    Polite = 1,
    Passive = 2,
    Causative = 3,
    Continuous = 4,
    Desire = 5,
    Negation = 6,
    Past = 7,
    Te = 8,
}

#[derive(Debug, PartialEq, PartialOrd, Default, Deserialize, Clone)]
// example verb: 話す
pub struct Verb {
    pub furigana: String, // はな
    pub base: String,     // 話
    pub ender: String,    // す
    pub kind: VerbKind,   // godan
}

#[derive(Debug, PartialEq, PartialOrd, Default, Deserialize, Clone)]
pub enum VerbKind {
    #[default]
    Ichidan,
    Godan,
    Irregular,
}

impl fmt::Display for ConjugationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConjugationKind::Polite => write!(f, "Polite"),
            ConjugationKind::Passive => write!(f, "Passive"),
            ConjugationKind::Causative => write!(f, "Causative"),
            ConjugationKind::Continuous => write!(f, "Continuous"),
            ConjugationKind::Desire => write!(f, "たい form"),
            ConjugationKind::Negation => write!(f, "Negative"),
            ConjugationKind::Past => write!(f, "Past"),
            ConjugationKind::Te => write!(f, "て form"),
        }
    }
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
            kind: VerbKind::Godan,
        }
    }

    pub fn ichidan(base: &str, ender: &str, furigana: &str) -> Self {
        Verb {
            furigana: furigana.to_string(),
            base: base.to_string(),
            ender: ender.to_string(),
            kind: VerbKind::Ichidan,
        }
    }

    pub fn irregular(base: &str, ender: &str, furigana: &str) -> Self {
        Verb {
            furigana: furigana.to_string(),
            base: base.to_string(),
            ender: ender.to_string(),
            kind: VerbKind::Irregular,
        }
    }

    pub fn conjugate(
        &mut self,
        transformations: &mut Vec<ConjugationKind>,
    ) -> Result<&mut Self, ConjugationError> {
        transformations.sort();

        let mut previous_conjugation = None;
        for transformation in transformations {
            self.apply_conjunction(&transformation, previous_conjugation)?;
            previous_conjugation = Some(transformation);
        }

        Ok(self)
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
            ConjugationKind::Causative => {
                self.causative(previous);
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

    fn causative(&mut self, previous: Option<&ConjugationKind>) {
        let conjugation_str = "せる";
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
                if self.kind == VerbKind::Godan {
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
                if self.kind == VerbKind::Irregular {
                    if self.ender.ends_with("する") {
                        self.ender = self.ender.replace("する", "し");
                    }
                } else if self.kind == VerbKind::Godan {
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

        if self.kind == VerbKind::Godan {
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
        let conjugation_str = "いる";

        match previous {
            None => {
                if self.kind == VerbKind::Godan {
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
                    self.ender.push_str("て");
                }
            }

            Some(ConjugationKind::Negation) | Some(ConjugationKind::Desire) => {
                // Consume い
                self.ender.pop();
                self.ender.push_str("くて")
            }
            Some(ConjugationKind::Passive) => {
                // Consume る
                self.ender.pop();
                self.ender.push_str("て");
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
                if self.kind == VerbKind::Godan {
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
            _ => panic!(
                "Incorrect grammar, wrong conjugation precedence, previous: {:?}",
                previous
            ),
        }

        self.ender.push_str(conjugation_str);
    }
}

#[cfg(test)]
#[test]
fn verb_su() {
    let verb = Verb::godan("話", "す", "はな");

    // Present
    let mut c = vec![];
    assert_eq!(verb.clone().conjugate(&mut c).unwrap().get_word(), "話す");

    // Present negative
    let mut c = vec![ConjugationKind::Negation];
    assert_eq!(
        verb.clone().conjugate(&mut c).unwrap().get_word(),
        "話さない"
    );

    // Past
    let mut c = vec![ConjugationKind::Past];
    assert_eq!(verb.clone().conjugate(&mut c).unwrap().get_word(), "話した");

    // Past negative
    let mut c = vec![ConjugationKind::Negation, ConjugationKind::Past];
    assert_eq!(
        verb.clone().conjugate(&mut c).unwrap().get_word(),
        "話さなかった"
    );

    // Te form
    let mut c = vec![ConjugationKind::Te];
    assert_eq!(verb.clone().conjugate(&mut c).unwrap().get_word(), "話して");

    // Negative Te form
    c = vec![ConjugationKind::Te, ConjugationKind::Negation];
    assert_eq!(
        verb.clone().conjugate(&mut c).unwrap().get_word(),
        "話さなくて"
    );

    // Present Tai form
    c = vec![ConjugationKind::Desire];
    assert_eq!(
        verb.clone().conjugate(&mut c).unwrap().get_word(),
        "話したい"
    );

    // Past Tai form
    c = vec![ConjugationKind::Past, ConjugationKind::Desire];
    assert_eq!(
        verb.clone().conjugate(&mut c).unwrap().get_word(),
        "話したかった"
    );

    // Present Negative Tai form
    c = vec![ConjugationKind::Negation, ConjugationKind::Desire];
    assert_eq!(
        verb.clone().conjugate(&mut c).unwrap().get_word(),
        "話したくない"
    );

    // Past Negative Tai form
    c = vec![
        ConjugationKind::Past,
        ConjugationKind::Negation,
        ConjugationKind::Desire,
    ];
    assert_eq!(
        verb.clone().conjugate(&mut c).unwrap().get_word(),
        "話したくなかった"
    );

    // Te form & Tai form
    c = vec![ConjugationKind::Te, ConjugationKind::Desire];
    assert_eq!(
        verb.clone().conjugate(&mut c).unwrap().get_word(),
        "話したくて"
    );

    // Negative Te form & Tai form
    c = vec![
        ConjugationKind::Negation,
        ConjugationKind::Te,
        ConjugationKind::Desire,
    ];
    assert_eq!(
        verb.clone().conjugate(&mut c).unwrap().get_word(),
        "話したくなくて"
    );

    // Negative Te form & Tai form
    c = vec![
        ConjugationKind::Negation,
        ConjugationKind::Te,
        ConjugationKind::Desire,
    ];
    assert_eq!(
        verb.clone().conjugate(&mut c).unwrap().get_word(),
        "話したくなくて"
    );

    // TODO: Volitional 話そう
    // TODO: Adverbial 話したく
    // TODO: Conditional 話したければ
}
