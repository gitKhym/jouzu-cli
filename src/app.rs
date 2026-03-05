use std::{
    io,
    iter::{self, empty},
};

use crossterm::event::KeyModifiers;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text, ToSpan},
    widgets::{Block, BorderType, List, Padding, Paragraph, Wrap},
};
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;
use wana_kana::ConvertJapanese;

use crate::game::{Feedback, Game};

#[derive(Debug, Default)]
pub struct App {
    input: Input,
    game_state: GameState,
    game: Game,
    feedback: Option<Feedback>, // TODO: Feedback should be in game
    show_furigana: bool,        // TODO: Create settings/config
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum GameState {
    Normal,
    #[default]
    Answering,
    Reviewing,
}

impl App {
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.game = Game::new();
        self.game.new_round();

        loop {
            terminal.draw(|frame| self.render(frame))?;

            let event = event::read()?;
            if let Event::Key(key) = event {
                if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    return Ok(());
                }

                match self.game_state {
                    GameState::Normal => match key.code {
                        KeyCode::Enter => self.start_editing(),
                        _ => {}
                    },
                    GameState::Answering => match key.code {
                        KeyCode::Enter => self.submit(),
                        KeyCode::Char('?') => self.new_round(),
                        KeyCode::Char('F') => self.toggle_show_furigana(),
                        // KeyCode::Esc => self.stop_editing(),
                        _ => {
                            self.input.handle_event(&event);
                        }
                    },
                    GameState::Reviewing => match key.code {
                        KeyCode::Enter => self.new_round(),
                        KeyCode::Char('F') => self.toggle_show_furigana(),
                        // KeyCode::Esc => self.stop_editing(),
                        KeyCode::Char('r') => self.reset(),
                        _ => {}
                    },
                }
            }
        }
    }

    fn toggle_show_furigana(&mut self) {
        self.show_furigana = !self.show_furigana;
    }

    fn new_round(&mut self) {
        self.game.new_round();
        self.feedback = None;
        self.input = Input::default();
        self.game_state = GameState::Answering;
    }

    fn reset(&mut self) {
        self.game = Game::new();
        self.new_round();
    }

    fn start_editing(&mut self) {
        self.game_state = GameState::Answering
    }

    fn stop_editing(&mut self) {
        self.game_state = GameState::Normal
    }

    fn submit(&mut self) {
        let input = self.input.value_and_reset();

        match self.game.check_answer(&input) {
            Ok(feedback) => {
                self.feedback = Some(feedback);
                self.input = Input::new(input.to_hiragana());
                self.game_state = GameState::Reviewing;
            }
            Err(_) => {}
        }
    }

    fn render(&self, frame: &mut Frame) {
        let terminal_area = frame.area();

        let [main_area, footer_area] =
            Layout::vertical([Constraint::Min(0), Constraint::Length(2)]).areas(terminal_area);

        let [_, centered_area, _] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Min(35),
            Constraint::Fill(1),
        ])
        .areas(main_area);

        let [_, target_area, vocab_area, input_area, performance_panel, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(centered_area);

        self.render_hotkeys(frame, footer_area);

        self.render_target_conjugation(frame, target_area);
        self.render_vocab(frame, vocab_area);
        self.render_input(frame, input_area);
        self.render_performance_panel(frame, performance_panel);
        self.render_hotkeys(frame, footer_area);
    }

    fn render_input(&self, frame: &mut Frame, area: Rect) {
        let width = area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);
        let input = Paragraph::new(self.input.value())
            .scroll((0, scroll as u16))
            .block(Block::bordered().border_type(BorderType::Rounded));
        frame.render_widget(input, area);

        if self.game_state == GameState::Answering {
            let x = self.input.visual_cursor().max(scroll) - scroll + 1;
            frame.set_cursor_position((area.x + x as u16, area.y + 1))
        }
    }

    fn render_vocab(&self, frame: &mut Frame, area: Rect) {
        let style = match self.feedback {
            Some(Feedback::Correct) => Style::default().fg(Color::Green),
            Some(Feedback::Incorrect(_)) => Style::default().fg(Color::LightRed),
            None => Style::default(),
        };

        let vocab = Text::from(Line::from(
            empty()
                .chain(std::iter::once(Span::from(self.game.round.word_base())))
                .chain(
                    self.show_furigana
                        .then(|| Span::from(format!("({})", self.game.round.word_furigana()))),
                )
                .chain(std::iter::once(Span::styled(
                    self.game.round.word_ending(),
                    style,
                )))
                .collect::<Vec<Span>>(),
        ));

        let vocab_area = Paragraph::new(vocab)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title(Line::from(" Conjugate ").left_aligned())
                    .title(Line::from(format!(" {} ", self.game.round.verb_type())).right_aligned())
                    .title_alignment(Alignment::Right),
            );

        frame.render_widget(vocab_area, area);
    }

    fn render_target_conjugation(&self, frame: &mut Frame, area: Rect) {
        let target_area = Paragraph::new(self.game.round.format_conjugations())
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(target_area, area);
    }

    fn render_performance_panel(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let score = self.score_component().alignment(Alignment::Left);

        frame.render_widget(score, chunks[0]);

        if let Some(feedback) = self.feedback_component() {
            let feedback = feedback.alignment(Alignment::Right);
            frame.render_widget(feedback, chunks[1]);
        }
    }

    fn score_component<'a>(&self) -> Paragraph<'a> {
        let score = self.game.correct_count;
        let attempts = self.game.attempts;

        Paragraph::new(format!("{}/{}", score.to_string(), attempts.to_string()))
    }

    fn feedback_component<'a>(&self) -> Option<Paragraph<'a>> {
        let style = match self.feedback {
            Some(Feedback::Correct) => Style::default().fg(Color::Green),
            Some(Feedback::Incorrect(_)) => Style::default().fg(Color::LightRed),
            None => Style::default(),
        };
        let Some(feedback) = &self.feedback else {
            return None;
        };

        let text = match feedback {
            Feedback::Correct => "Correct",
            Feedback::Incorrect(_) => "Incorrect",
        };

        Some(
            Paragraph::new(text)
                .alignment(Alignment::Center)
                .style(style),
        )
    }

    fn render_hotkeys(&self, frame: &mut Frame, area: Rect) {
        let hints = match self.game_state {
            GameState::Normal => hints(vec![("^c", "quit"), ("?", "hints"), ("r", "reset")]),
            GameState::Answering => hints(vec![
                ("^c", "quit"),
                ("?", "hints"),
                (
                    "F",
                    if self.show_furigana {
                        "hide furigana"
                    } else {
                        "show furigana"
                    },
                ),
            ]),
            GameState::Reviewing => hints(vec![
                ("^c", "quit"),
                ("?", "hints"),
                ("r", "reset"),
                (
                    "F",
                    if self.show_furigana {
                        "hide furigana"
                    } else {
                        "show furigana"
                    },
                ),
                ("enter", "next"),
            ]),
        };

        let hint_area = Paragraph::new(Line::from(hints)).alignment(Alignment::Center);
        frame.render_widget(hint_area, area);
    }
}

fn hints<'a>(hints: Vec<(&str, &str)>) -> Vec<Span<'a>> {
    let mut spans = Vec::new();
    for (i, (key, action)) in hints.iter().enumerate() {
        spans.push(Span::styled(
            key.to_string(),
            Style::default()
                .fg(Color::Rgb(136, 136, 136))
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::styled(" ".to_string(), Style::default()));
        spans.push(Span::styled(action.to_string(), Style::default()));
        if i != hints.len() - 1 {
            spans.push(Span::raw(" · "));
        }
    }
    spans
}
