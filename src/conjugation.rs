use core::fmt;
use std::marker::PhantomData;

use serde::Deserialize;

use crate::{
    utils::{is_voiced, to_a, to_e, to_i},
    verb::{ConjugationKind, VerbKind},
};

#[derive(Debug, PartialEq, PartialOrd, Default, Deserialize, Clone)]
pub enum WordKind {
    #[default]
    Ichidan,
    Godan,
    IAdjective,
    Irregular,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Copy, Clone)]
pub struct VerbStem;
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Copy, Clone)]
pub struct AdjStem;
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Copy, Clone)]
pub struct Nominal;
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Copy, Clone)]
pub struct Conclusive;

#[derive(Debug, PartialEq, PartialOrd, Default, Deserialize, Clone)]
pub struct Word<State> {
    pub furigana: String, // はな
    pub base: String,     // 話
    pub ender: String,    // す
    pub kind: WordKind,   // godan
    _state: PhantomData<State>,
}

impl<S> Word<S> {
    /// Internal helper to switch states without rebuilding the whole struct
    fn transition<NewState>(self) -> Word<NewState> {
        Word {
            base: self.base,
            furigana: self.furigana,
            ender: self.ender,
            kind: self.kind,
            _state: std::marker::PhantomData,
        }
    }
}

impl<S> fmt::Display for Word<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.base, self.ender)
    }
}

impl Word<VerbStem> {
    pub fn godan(base: &str, furigana: &str, ender: &str) -> Self {
        Self {
            base: base.to_string(),
            furigana: furigana.to_string(),
            ender: ender.to_string(),
            kind: WordKind::Godan,
            _state: PhantomData,
        }
    }

    pub fn ichidan(base: &str, furigana: &str, ender: &str) -> Self {
        Self {
            base: base.to_string(),
            furigana: furigana.to_string(),
            ender: ender.to_string(),
            kind: WordKind::Ichidan,
            _state: PhantomData,
        }
    }
}

impl Word<AdjStem> {
    pub fn i_adj(base: &str, furigana: &str, ender: &str) -> Self {
        Self {
            base: base.to_string(),
            furigana: furigana.to_string(),
            ender: ender.to_string(),
            kind: WordKind::IAdjective,
            _state: PhantomData,
        }
    }
}

impl Word<VerbStem> {
    pub fn negate(mut self) -> Word<AdjStem> {
        match self.kind {
            WordKind::Godan => {
                if let Some(last) = self.ender.pop() {
                    self.ender.push(to_a(&last));
                }
            }

            WordKind::Ichidan => {
                self.ender.pop();
            }

            WordKind::IAdjective => {
                self.ender.pop();
                self.ender.push_str("くない");
            }

            WordKind::Irregular => {
                if self.ender.ends_with("する") {
                    self.ender = self.ender.replace("する", "し");
                }
            }
        }
        self.ender.push_str("ない");
        self.transition()
    }

    pub fn past(mut self) -> Word<Conclusive> {
        match self.kind {
            WordKind::Godan => {
                if let Some(last) = self.ender.pop() {
                    match last {
                        'る' | 'う' | 'つ' => self.ender.push('っ'),
                        'ぬ' | 'ぶ' | 'む' => self.ender.push('ん'),
                        'く' | 'ぐ' => self.ender.push('い'),
                        'す' => self.ender.push('し'),
                        _ => panic!("Invalid verb ending: {}", last),
                    }
                }
            }

            WordKind::Ichidan => {
                self.ender.pop();
            }

            WordKind::IAdjective => {}

            WordKind::Irregular => {
                if self.ender.ends_with("する") {
                    self.ender = self.ender.replace("する", "し");
                }
            }
        }

        self.ender
            .push_str(if is_voiced(&self.ender) { "だ" } else { "た" });
        self.kind = WordKind::Ichidan;
        self.transition()
    }

    pub fn te(mut self) -> Word<Conclusive> {
        let suffix = if is_voiced(&self.ender) { "で" } else { "て" };
        match self.kind {
            WordKind::Godan => {
                if let Some(last) = self.ender.pop() {
                    match last {
                        'る' | 'う' | 'つ' => self.ender.push('っ'),
                        'ぬ' | 'ぶ' | 'む' => self.ender.push('ん'),
                        'く' | 'ぐ' => self.ender.push('い'),
                        'す' => self.ender.push('し'),
                        _ => panic!("Invalid verb ending: {}", last),
                    }
                }
            }

            WordKind::Ichidan => {
                self.ender.pop();
            }

            WordKind::IAdjective => {}

            WordKind::Irregular => {
                if self.ender.ends_with("する") {
                    self.ender = self.ender.replace("する", "し");
                }
            }
        }

        self.ender.push_str(suffix);
        self.kind = WordKind::Ichidan;
        self.transition()
    }

    pub fn passive(mut self) -> Word<VerbStem> {
        match self.kind {
            WordKind::Godan | WordKind::Ichidan => {
                if let Some(last) = self.ender.pop() {
                    self.ender.push(to_a(&last));
                }
            }
            _ => todo!("For passive: IAdjectives and Irregular"),
        }

        self.ender.push_str("れる");

        self.kind = WordKind::Ichidan;
        self.transition()
    }

    pub fn causative(mut self) -> Word<VerbStem> {
        match self.kind {
            WordKind::Godan | WordKind::Ichidan => {
                if let Some(last) = self.ender.pop() {
                    self.ender.push(to_a(&last));
                }
            }
            _ => todo!("For causative: IAdjectives and Irregular"),
        }

        self.ender.push_str("せる");

        self.kind = WordKind::Ichidan;
        self.transition()
    }

    pub fn potential(mut self) -> Word<VerbStem> {
        match self.kind {
            WordKind::Godan | WordKind::Ichidan => {
                if let Some(last) = self.ender.pop() {
                    self.ender.push(to_e(&last));
                }
            }
            WordKind::IAdjective => todo!(),
            WordKind::Irregular => todo!(),
        }

        self.ender.push_str("る");
        self.kind = WordKind::Ichidan;
        self.transition()
    }

    pub fn desire(mut self) -> Word<AdjStem> {
        match self.kind {
            WordKind::Godan => {
                if let Some(last) = self.ender.pop() {
                    self.ender.push(to_i(&last));
                }
            }

            WordKind::Ichidan => {
                self.ender.pop();
            }

            WordKind::IAdjective => {
                self.ender.pop();
                self.ender.push_str("くない");
            }

            WordKind::Irregular => {
                if self.ender.ends_with("する") {
                    self.ender = self.ender.replace("する", "し");
                }
            }
        }
        self.ender.push_str("たい");
        self.kind = WordKind::IAdjective;
        self.transition()
    }

    pub fn imperative(mut self) -> Word<Conclusive> {
        match self.kind {
            WordKind::Godan => {
                if let Some(last) = self.ender.pop() {
                    self.ender.push(to_e(&last));
                }
            }
            WordKind::Ichidan => {
                self.ender.pop();
                self.ender.push_str("ろ");
            }
            WordKind::IAdjective => todo!(),
            WordKind::Irregular => todo!(),
        }
        self.transition()
    }
    pub fn provisional(mut self) -> Word<Conclusive> {
        match self.kind {
            WordKind::Godan | WordKind::Ichidan => {
                if let Some(last) = self.ender.pop() {
                    self.ender.push(to_e(&last));
                }
            }
            WordKind::IAdjective => todo!(),
            WordKind::Irregular => todo!(),
        }
        self.ender.push_str("ば");
        self.transition()
    }
}

impl Word<AdjStem> {
    pub fn negate(mut self) -> Word<AdjStem> {
        self.ender.pop();
        self.ender.push_str("くない");
        self.transition()
    }

    pub fn te(mut self) -> Word<Conclusive> {
        self.ender.pop();
        self.ender.push_str("くて");
        self.transition()
    }

    pub fn past(mut self) -> Word<Conclusive> {
        self.ender.pop();
        self.ender.push_str("かった");
        self.transition()
    }

    pub fn provisional(mut self) -> Word<Conclusive> {
        self.ender.pop();
        self.ender.push_str("ければ");
        self.transition()
    }
    pub fn conditional(mut self) -> Word<Conclusive> {
        self.ender.pop();
        self.ender.push_str("かったら");
        self.transition()
    }

    pub fn objective(mut self) -> Word<Conclusive> {
        self.ender.pop();
        self.ender.push_str("さ");
        self.transition()
    }
}

#[cfg(test)]
#[test]
fn godan_verb_ending() {
    let base = Word::godan("話", "はな", "す");

    // -- PLAIN --
    // Negative
    let v = base.clone();
    assert_eq!(v.negate().to_string(), "話さない".to_string());

    // Past
    let v = base.clone();
    assert_eq!(v.past().to_string(), "話した".to_string());

    // Negative Past
    let v = base.clone();
    assert_eq!(v.negate().past().to_string(), "話さなかった".to_string());

    // -- TAI FORM --
    // Tai
    let v = base.clone();
    assert_eq!(v.desire().to_string(), "話したい".to_string());

    // Negative Tai
    let v = base.clone();
    assert_eq!(v.desire().negate().to_string(), "話したくない".to_string());

    // Past Tai
    let v = base.clone();
    assert_eq!(v.desire().past().to_string(), "話したかった".to_string());

    // Negative Past Tai
    let v = base.clone();
    assert_eq!(
        v.desire().negate().past().to_string(),
        "話したくなかった".to_string()
    );

    // Tai Te
    let v = base.clone();
    assert_eq!(v.desire().te().to_string(), "話したくて".to_string());

    // Negative Tai Te
    let v = base.clone();
    assert_eq!(
        v.desire().negate().te().to_string(),
        "話したくなくて".to_string()
    );

    // -- POTENTIAL --
    let v = base.clone();
    assert_eq!(v.potential().to_string(), "話せる".to_string());

    let v = base.clone();
    assert_eq!(v.potential().negate().to_string(), "話せない".to_string());

    let v = base.clone();
    assert_eq!(v.potential().past().to_string(), "話せた".to_string());

    let v = base.clone();
    assert_eq!(
        v.potential().negate().past().to_string(),
        "話せなかった".to_string()
    );

    let v = base.clone();
    assert_eq!(v.potential().te().to_string(), "話せて".to_string());

    let v = base.clone();
    assert_eq!(
        v.potential().negate().te().to_string(),
        "話せなくて".to_string()
    );

    // -- TE --
    // Base
    let v = base.clone();
    assert_eq!(v.negate().te().to_string(), "話さなくて".to_string());
}

#[test]
fn te_endings() {
    let cases = [
        ("う", "って"),
        ("つ", "って"),
        ("る", "って"),
        ("む", "んで"),
        ("ぶ", "んで"),
        ("ぬ", "んで"),
        ("く", "いて"),
        ("ぐ", "いで"),
        ("す", "して"),
    ];

    for (ender, expected) in cases {
        let word = Word::godan("", "", ender);
        let te = word.te();
        assert_eq!(te.ender, expected);
    }
}
