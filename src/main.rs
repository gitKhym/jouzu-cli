use lexicon::{Conjugatable, Verb};
mod input;
mod lexicon;
mod utils;

fn main() {
    let verb_ichidan = Verb {
        word: "食べる".into(),
        furigana: "たべる".into(),
        root: "食べ".into(),
        ender: "る".into(),
        is_ichidan: true,
    };

    let verb_godan = Verb {
        word: "話す".into(),
        furigana: "はなす".into(),
        root: "話".into(),
        ender: "す".into(),
        is_ichidan: false,
    };

    let modified_ichidan = verb_ichidan.polite();
    println!("{modified_ichidan:?}");
    let modified_godan = verb_godan.polite();
    println!("{modified_godan:?}")
}
