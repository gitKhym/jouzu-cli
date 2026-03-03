use crate::utils::{is_voiced, last, to_a, to_i};

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Copy, Clone)]
pub enum ConjugationKind {
    Passive = 0,
    Continuous = 2,
    Desire = 3,
    Negation = 5,
    Past = 6,
    Te = 7,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Verb {
    // TODO: furigana
    // TODO: applied_conjugations
    base_form: String,
    is_godan: bool,
}

#[derive(Debug)]
pub enum ConjugationError {
    InvalidCombinations(String),
}

impl Verb {
    pub fn godan(base: &str) -> Self {
        Verb {
            base_form: base.to_string(),
            is_godan: true,
        }
    }

    pub fn ichidan(base: &str) -> Self {
        Verb {
            base_form: base.to_string(),
            is_godan: false,
        }
    }

    pub fn conjugate(
        &self,
        mut transformations: Vec<ConjugationKind>,
    ) -> Result<String, ConjugationError> {
        transformations.sort();
        println!("{:?}", transformations);

        let mut curr = self.base_form.clone();
        let mut previous: Option<ConjugationKind> = None;

        for t in transformations {
            curr = Self::apply(self, curr, t, previous)?;
            previous = Some(t);
        }

        Ok(curr)
    }

    fn apply(
        verb: &Verb,
        current: String,
        kind: ConjugationKind,
        previous: Option<ConjugationKind>,
    ) -> Result<String, ConjugationError> {
        match kind {
            ConjugationKind::Negation => Ok(Self::negative(verb, current, previous)),
            ConjugationKind::Past => Ok(Self::past(verb, current, previous)),
            ConjugationKind::Desire => Ok(Self::desire(verb, current, previous)),
            ConjugationKind::Passive => Ok(Self::passive(verb, current, previous)),
            ConjugationKind::Te => Self::te_form(verb, current, previous).map_err(|_| {
                ConjugationError::InvalidCombinations(
                    "Te-form cannot be paired with Past form".to_string(),
                )
            }),
            ConjugationKind::Continuous => Ok(Self::continous(verb, current, previous)),
        }
    }
    fn passive(verb: &Verb, mut current: String, previous: Option<ConjugationKind>) -> String {
        let conjugation_str = "れる";
        match previous {
            None => {
                let last = last(&current);
                current.pop();
                current.push(to_a(&last));
            }
            _ => panic!("Incorrect grammar, wrong conjugation precedence"),
        }

        current.push_str(conjugation_str);
        current
    }

    fn negative(verb: &Verb, mut current: String, previous: Option<ConjugationKind>) -> String {
        let conjugation_str = "ない";

        match previous {
            None => {
                if verb.is_godan {
                    let last = last(&current);
                    current.pop();
                    current.push(to_a(&last));
                } else {
                    current.pop();
                }
            }
            Some(ConjugationKind::Desire) => {
                // Consume い
                current.pop();
                current.push_str("く")
            }
            Some(ConjugationKind::Passive) | Some(ConjugationKind::Continuous) => {
                // Consume る
                current.pop();
            }
            _ => panic!("Incorrect grammar, wrong conjugation precedence"),
        }

        current.push_str(conjugation_str);
        current
    }

    fn past(verb: &Verb, mut current: String, previous: Option<ConjugationKind>) -> String {
        let conjugation_str = if is_voiced(&current) { "だ" } else { "た" };

        match previous {
            None => {
                if verb.is_godan {
                    let last = last(&current);
                    current.pop();

                    match last {
                        'る' | 'う' | 'つ' => current.push('っ'),
                        'ぬ' | 'ぶ' | 'む' => current.push('ん'),
                        'く' | 'ぐ' => current.push('い'),
                        'す' => current.push('し'),
                        _ => panic!("Invalid verb ending: {}", last),
                    }
                } else {
                    current.pop();
                }
            }
            Some(ConjugationKind::Negation) | Some(ConjugationKind::Desire) => {
                // Consume い
                current.pop();
                current.push_str("かっ")
            }
            Some(ConjugationKind::Passive) | Some(ConjugationKind::Continuous) => {
                // Consume る
                current.pop();
            }
            _ => panic!(
                "Incorrect grammar, wrong conjugation precedence, previous: {:?}",
                previous
            ),
        }

        current.push_str(conjugation_str);
        current
    }

    fn desire(verb: &Verb, mut current: String, previous: Option<ConjugationKind>) -> String {
        let conjugation_str = "たい";

        if verb.is_godan {
            match previous {
                None => {
                    let last = last(&current);
                    current.pop();
                    current.push(to_i(&last));
                }
                Some(ConjugationKind::Negation) => {
                    // Consume い
                    current.pop();
                    current.push_str("かっ")
                }
                Some(ConjugationKind::Passive) | Some(ConjugationKind::Continuous) => {
                    // Consume る
                    current.pop();
                }
                _ => panic!(
                    "Incorrect grammar, wrong conjugation precedence, previous: {:?}",
                    previous
                ),
            }
        } else {
            current.pop();
        }

        current.push_str(conjugation_str);
        current
    }

    fn continous(verb: &Verb, mut current: String, previous: Option<ConjugationKind>) -> String {
        let mut conjugation_str = "いる";

        match previous {
            None => {
                if verb.is_godan {
                    conjugation_str = "いる";

                    let last = last(&current);
                    current.pop();

                    match last {
                        'ぬ' | 'ぶ' | 'む' => current.push_str("んで"),
                        'ぐ' => current.push_str("いで"),
                        'る' | 'う' | 'つ' => current.push_str("って"),
                        'く' => current.push_str("いて"),
                        'す' => current.push_str("して"),
                        _ => panic!("Invalid verb ending: {}", last),
                    }
                } else {
                    current.pop();
                }
            }

            Some(ConjugationKind::Negation) | Some(ConjugationKind::Desire) => {
                // Consume い
                current.pop();
                current.push_str("く")
            }
            Some(ConjugationKind::Passive) => {
                // Consume る
                current.pop();
            }

            _ => panic!(
                "Incorrect grammar, wrong conjugation precedence, previous: {:?}",
                previous
            ),
        }

        current.push_str(conjugation_str);
        current
    }

    fn te_form(
        verb: &Verb,
        mut current: String,
        previous: Option<ConjugationKind>,
    ) -> Result<String, ConjugationError> {
        let mut conjugation_str = "て";

        match previous {
            None => {
                if verb.is_godan {
                    conjugation_str = "で";

                    let last = last(&current);
                    current.pop();

                    match last {
                        'る' | 'う' | 'つ' => current.push('っ'),
                        'ぬ' | 'ぶ' | 'む' => current.push('ん'),
                        'く' | 'ぐ' => current.push('い'),
                        'す' => current.push('し'),
                        _ => panic!("Invalid verb ending: {}", last),
                    }
                } else {
                    current.pop();
                }
            }

            Some(ConjugationKind::Negation) | Some(ConjugationKind::Desire) => {
                // Consume い
                current.pop();
                current.push_str("く")
            }
            Some(ConjugationKind::Passive) => {
                // Consume る
                current.pop();
            }

            Some(ConjugationKind::Past | ConjugationKind::Continuous) => {
                let msg = match previous.unwrap() {
                    ConjugationKind::Past => "Te form cannot be combined with Past form",
                    ConjugationKind::Continuous => {
                        "Te form cannot be combined with Continuous form"
                    }
                    _ => unreachable!(),
                };
                return Err(ConjugationError::InvalidCombinations(msg.to_string()));
            }

            _ => panic!(
                "Incorrect grammar, wrong conjugation precedence, previous: {:?}",
                previous
            ),
        }

        current.push_str(conjugation_str);
        Ok(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn godan(base: &str) -> Verb {
        Verb {
            base_form: base.to_string(),
            is_godan: true,
        }
    }

    fn ichidan(base: &str) -> Verb {
        Verb {
            base_form: base.to_string(),
            is_godan: false,
        }
    }

    #[test]
    fn su_godan_hanasu() {
        let verb = godan("話す");

        assert_eq!(
            verb.conjugate(vec![ConjugationKind::Negation]).unwrap(),
            "話さない"
        );
        assert_eq!(
            verb.conjugate(vec![ConjugationKind::Past]).unwrap(),
            "話した"
        );
        assert_eq!(
            verb.conjugate(vec![ConjugationKind::Desire]).unwrap(),
            "話したい"
        );

        assert_eq!(
            verb.conjugate(vec![ConjugationKind::Negation, ConjugationKind::Past])
                .unwrap(),
            "話さなかった"
        );

        assert_eq!(
            verb.conjugate(vec![ConjugationKind::Desire, ConjugationKind::Negation])
                .unwrap(),
            "話したくない"
        );

        assert_eq!(
            verb.conjugate(vec![ConjugationKind::Desire, ConjugationKind::Past])
                .unwrap(),
            "話したかった"
        );

        assert_eq!(
            verb.conjugate(vec![
                ConjugationKind::Desire,
                ConjugationKind::Negation,
                ConjugationKind::Past
            ])
            .unwrap(),
            "話したくなかった"
        );
    }
}
