use app::App;

use crate::conjugation::Word;
mod app;
mod conjugation;
mod game;
mod round;
mod utils;
mod verb;

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}
// fn main() {
//     let word = Word::godan("話", "はな", "す");
//
//     let x = word.desire().provisional().to_string();
//     println!("{:?}", x);
// }
