use crate::utils::{to_a, to_i};

pub trait Conjugatable {
    fn negate(self) -> Self;
    fn past(self) -> Self;
    fn polite(self) -> Self;
    fn te_form(self) -> Self;
    fn volitional(self) -> Self;
    fn causative(self) -> Self;
}
// ichidan:
// root: 開け, ender: る
// godan:
// root: 話, ender: す
#[derive(Debug)]
pub struct Verb {
    pub word: String,
    pub furigana: String,
    pub root: String,
    pub ender: String,
    pub is_ichidan: bool,
}

impl Conjugatable for Verb {
    fn negate(mut self) -> Self {
        if self.is_ichidan {
            self.ender = "ない".into();
            self.word = format!("{}ない", self.root);
        } else {
            self.ender = to_a(&self.ender.as_str());
            self.word = format!("{}{}ない", self.root, self.ender);
        }
        self
    }
    fn past(mut self) -> Self {
        if self.is_ichidan {
            self.ender = "た".into();
            self.word = format!("{}{}", self.root, self.ender);
        } else {
            self.ender = format!("{}{}", to_i(&self.ender.as_str()), "た");
            self.word = format!("{}{}", self.root, self.ender);
        }
        self
    }

    fn polite(mut self) -> Self {
        if self.is_ichidan {
            self.ender = "ます".into();
            self.word = format!("{}{}", self.root, self.ender);
        } else {
            self.ender = format!("{}{}", to_i(&self.ender.as_str()), "ます");
            self.word = format!("{}{}", self.root, self.ender);
        }
        self
    }

    fn te_form(mut self) -> Self {
        todo!()
    }

    fn volitional(mut self) -> Self {
        todo!()
    }

    fn causative(mut self) -> Self {
        todo!()
    }
}
