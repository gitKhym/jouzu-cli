mod utils;
mod verb;
use verb::{ConjugationError, ConjugationKind, Verb};

fn main() {
    let verb = Verb::godan("買う");

    let conjugations = vec![
        ConjugationKind::Passive,
        ConjugationKind::Continuous,
        ConjugationKind::Desire,
        ConjugationKind::Negation,
        ConjugationKind::Past,
        ConjugationKind::Te,
    ];

    for i in 0..conjugations.len() {
        for j in (i + 1)..conjugations.len() {
            let combo = vec![conjugations[i], conjugations[j]];
            let result = verb.conjugate(combo);
            let output = match result {
                Ok(s) => s,
                Err(ConjugationError::InvalidCombinations(msg)) => msg.clone(),
            };
            println!(
                "{:?} + {:?} -> {}",
                conjugations[i], conjugations[j], output
            );
        }
    }
}
