use crate::utils::{ends_with_dakuten, is_voiced, last, to_a, to_i};

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Copy, Clone)]
pub enum ConjugationKind {
    Desire = 0,
    Negation = 5,
    Past = 6,
    Te = 7,
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Verb {
    // TODO: furigana
    // TODO: applied_conjugations
    base_form: String,
    is_godan: bool,
}

impl Verb {
    fn conjugate(&self, mut transformations: Vec<ConjugationKind>) -> String {
        transformations.sort();

        let mut curr = self.base_form.clone();
        let mut previous: Option<ConjugationKind> = None;

        for t in transformations {
            curr = Self::apply(self, curr, t, previous);
            previous = Some(t);
        }

        curr
    }
    fn apply(
        verb: &Verb,
        current: String,
        kind: ConjugationKind,
        previous: Option<ConjugationKind>,
    ) -> String {
        match kind {
            ConjugationKind::Negation => Self::negative(verb, current, previous),
            ConjugationKind::Past => Self::past(verb, current, previous),
            ConjugationKind::Desire => Self::desire(verb, current, previous),
            ConjugationKind::Te => Self::desire(verb, current, previous),
        }
    }

    fn negative(verb: &Verb, mut current: String, previous: Option<ConjugationKind>) -> String {
        let conjugation_str = "ない";

        if verb.is_godan {
            match previous {
                None => {
                    let last = last(&current);
                    current.pop();
                    current.push(to_a(&last));
                }
                Some(ConjugationKind::Desire) => {
                    // Consume い
                    current.pop();
                    current.push_str("く")
                }
                _ => unreachable!("Incorrect grammar"),
            }
        } else {
            current.pop();
        }

        current.push_str(conjugation_str);
        current
    }

    fn past(verb: &Verb, mut current: String, previous: Option<ConjugationKind>) -> String {
        let conjugation_str = "た";

        if verb.is_godan {
            match previous {
                None => {
                    let last = last(&current);
                    current.pop();

                    if last == 'る' {
                        current.push('っ');
                    } else {
                        current.push(to_i(&last));
                    }
                }
                Some(ConjugationKind::Negation) | Some(ConjugationKind::Desire) => {
                    // Consume い
                    current.pop();
                    current.push_str("かっ")
                }
                _ => unreachable!("Incorrect grammar"),
            }
        } else {
            current.pop();
        }

        current.push_str(conjugation_str);
        current
    }

    fn desire(verb: &Verb, mut current: String, previous: Option<ConjugationKind>) -> String {
        let conjugation_str = "たい";

        if verb.is_godan {
            match previous {
                None => {
                    let last = Self::last(&current);
                    current.pop();
                    current.push(to_i(&last));
                }
                Some(ConjugationKind::Negation) => {
                    // Consume い
                    current.pop();
                    current.push_str("かっ")
                }
                _ => unreachable!("Incorrect grammar"),
            }
        } else {
            current.pop();
        }

        current.push_str(conjugation_str);
        current
    }
    fn te_form(verb: &Verb, mut current: String, previous: Option<ConjugationKind>) -> String {
        let conjugation_str = if is_voiced(&current) {
            String::from("で")
        } else {
            String::from("て")
        };
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

    #[test]
    fn su_godan_hanasu() {
        let verb = godan("話す");

        // single
        assert_eq!(verb.conjugate(vec![ConjugationKind::Negation]), "話さない");
        assert_eq!(verb.conjugate(vec![ConjugationKind::Past]), "話した");
        assert_eq!(verb.conjugate(vec![ConjugationKind::Desire]), "話したい");

        // double
        assert_eq!(
            verb.conjugate(vec![ConjugationKind::Negation, ConjugationKind::Past]),
            "話さなかった"
        );

        assert_eq!(
            verb.conjugate(vec![ConjugationKind::Desire, ConjugationKind::Negation]),
            "話したくない"
        );

        assert_eq!(
            verb.conjugate(vec![ConjugationKind::Desire, ConjugationKind::Past]),
            "話したかった"
        );

        // triple
        assert_eq!(
            verb.conjugate(vec![
                ConjugationKind::Desire,
                ConjugationKind::Negation,
                ConjugationKind::Past
            ]),
            "話したくなかった"
        );
    }

    #[test]
    fn ru_godan_toru() {
        let verb = godan("取る");

        // single
        assert_eq!(verb.conjugate(vec![ConjugationKind::Negation]), "取らない");
        assert_eq!(verb.conjugate(vec![ConjugationKind::Past]), "取った");
        assert_eq!(verb.conjugate(vec![ConjugationKind::Desire]), "取りたい");

        // double
        assert_eq!(
            verb.conjugate(vec![ConjugationKind::Negation, ConjugationKind::Past]),
            "取らなかった"
        );

        assert_eq!(
            verb.conjugate(vec![ConjugationKind::Desire, ConjugationKind::Negation]),
            "取りたくない"
        );

        assert_eq!(
            verb.conjugate(vec![ConjugationKind::Desire, ConjugationKind::Past]),
            "取りたかった"
        );

        // triple
        assert_eq!(
            verb.conjugate(vec![
                ConjugationKind::Desire,
                ConjugationKind::Negation,
                ConjugationKind::Past
            ]),
            "取りたくなかった"
        );
    }
}
