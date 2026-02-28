use lexicon::{Conjugatable, Verb};

use crate::lexicon::Conjugations;
mod input;
mod lexicon;
mod utils;

fn main() {
    let verb_ichidan = Verb {
        word: "食べる".into(),
        furigana: "たべる".into(),
        root: "食べ".into(),
        ender: "る".into(),
        conjugations: Conjugations {
            value: String::new(),
            kinds: Vec::new(),
        },
        is_godan: false,
    };

    let verb_godan = Verb {
        word: "話す".into(),
        furigana: "はなす".into(),
        root: "話".into(),
        ender: "す".into(),
        conjugations: Conjugations {
            value: String::new(),
            kinds: Vec::new(),
        },
        is_godan: true,
    };

    let verb_godan_2 = Verb {
        word: "飲む".into(),
        furigana: "はなす".into(),
        root: "飲".into(),
        ender: "む".into(),
        conjugations: Conjugations {
            value: String::new(),
            kinds: Vec::new(),
        },
        is_godan: true,
    };

    let modified_ichidan = verb_ichidan.desire().negate().past();
    println!("{modified_ichidan:?}");
    let modified_godan = verb_godan.desire().negate().past();
    println!("{modified_godan:?}");

    let modified_godan_2 = verb_godan_2.desire().past();
    println!("{modified_godan_2:?}")
}
