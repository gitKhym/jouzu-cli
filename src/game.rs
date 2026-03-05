use std::{fs::File, io::BufReader};

use rand::{Rng, seq::SliceRandom, thread_rng};
use serde_json::from_reader;
use wana_kana::ConvertJapanese;

use crate::{
    round::Round,
    verb::{ConjugationError, ConjugationKind, Verb},
};

#[derive(Debug, Default)]
pub struct Game {
    pub correct_count: u16,
    pub attempts: u16,
    pub active_conjugations: Vec<ConjugationKind>,
    pub round: Round,
}

#[derive(Debug)]
pub enum Feedback {
    Correct,
    Incorrect(String),
}

// TODO: Implement game as an iterator?
impl Game {
    pub fn new() -> Self {
        Self {
            correct_count: 0,
            attempts: 0,
            active_conjugations: vec![ConjugationKind::Negation],
            round: Round {
                word_to_conjugate: Verb::godan("", "", ""),
                conjugations_to_apply: Vec::new(),
            },
        }
    }

    pub fn new_round(&mut self) {
        self.round = Round {
            word_to_conjugate: Self::get_random_verb(),
            conjugations_to_apply: Self::get_random_conjugations(),
        }
    }

    pub fn get_random_verb() -> Verb {
        let file = File::open("word_bank.json").expect("Failed to open word_bank.json");
        let reader = BufReader::new(file);
        let verbs: Vec<Verb> = from_reader(reader).expect("Failed to parse JSON");
        let mut rng = thread_rng();
        verbs
            .choose(&mut rng)
            .expect("word_bank.json is empty")
            .clone()
    }

    // TODO: Disallow some combinations
    pub fn get_random_conjugations() -> Vec<ConjugationKind> {
        let mut rng = thread_rng();

        let mut pool = vec![
            ConjugationKind::Negation,
            ConjugationKind::Past,
            ConjugationKind::Te,
            ConjugationKind::Desire,
            ConjugationKind::Passive,
            ConjugationKind::Continuous,
        ];

        loop {
            pool.shuffle(&mut rng);
            let count = rng.gen_range(1..=2.min(pool.len()));
            let selection: Vec<_> = pool.iter().take(count).cloned().collect();

            if selection.contains(&ConjugationKind::Te)
                && selection.contains(&ConjugationKind::Past)
            {
                continue;
            }
            return selection;
        }
    }

    pub fn check_answer(&mut self, input: &str) -> Result<Feedback, ConjugationError> {
        // TODO: Don't use unwrap
        self.round
            .word_to_conjugate
            .conjugate(&mut self.round.conjugations_to_apply)
            .unwrap();

        self.attempts += 1;

        let hiragana_ans = self.round.word_to_conjugate.get_hiragana();
        if &input.to_hiragana() == &hiragana_ans {
            self.correct_count += 1;
            Ok(Feedback::Correct)
        } else {
            Ok(Feedback::Incorrect(self.round.word_to_conjugate.get_word()))
        }
    }
}
