use app::App;
mod app;
mod game;
mod round;
mod utils;
mod verb;

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}
