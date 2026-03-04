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
    pub correct: u16,
    pub active_conjugations: Vec<ConjugationKind>,
    pub round_num: usize,
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
            correct: 0,
            active_conjugations: vec![ConjugationKind::Negation],
            round_num: 0,
            round: Round {
                word_to_conjugate: Verb::godan("", "", ""),
                conjugations_to_apply: Vec::new(),
            },
        }
    }

    pub fn new_round(&mut self) {
        self.round_num += 1;

        self.round = Round {
            word_to_conjugate: Self::get_random_verb(),
            conjugations_to_apply: Self::get_random_conjugations(),
        }
    }

    // TODO: Implement getting random verb from a file
    pub fn get_random_verb() -> Verb {
        // Open the JSON file
        let file = File::open("word_bank.json").expect("Failed to open word_bank.json");
        let reader = BufReader::new(file);

        // Deserialize into a Vec<Verb>
        let verbs: Vec<Verb> = from_reader(reader).expect("Failed to parse JSON");

        // Pick a random verb
        let mut rng = thread_rng();
        verbs
            .choose(&mut rng)
            .expect("word_bank.json is empty")
            .clone()
    }

    // TODO: Implement getting random combination of conjugations from
    pub fn get_random_conjugations() -> Vec<ConjugationKind> {
        let mut rng = thread_rng();

        let mut pool = vec![
            ConjugationKind::Negation,
            ConjugationKind::Past,
            ConjugationKind::Te,
            ConjugationKind::Desire,
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
        // TODO: Incorrect, pass through correct string
        self.round
            .word_to_conjugate
            .conjugate(&mut self.round.conjugations_to_apply)
            .unwrap();

        let hiragana_ans = self.round.word_to_conjugate.get_hiragana();

        if &input.to_hiragana() != &hiragana_ans {
            Ok(Feedback::Incorrect(self.round.word_to_conjugate.get_word()))
        } else {
            Ok(Feedback::Correct)
        }
    }
}
