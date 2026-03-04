use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
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
    feedback: Option<Feedback>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum GameState {
    #[default]
    Normal,
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
                match self.game_state {
                    GameState::Normal => match key.code {
                        KeyCode::Char('e') => self.start_editing(),
                        KeyCode::Char('q') => return Ok(()),
                        _ => {}
                    },
                    GameState::Answering => match key.code {
                        KeyCode::Enter => self.submit(),
                        KeyCode::Esc => self.stop_editing(),
                        _ => {
                            self.input.handle_event(&event);
                        }
                    },
                    GameState::Reviewing => match key.code {
                        KeyCode::Enter => self.new_round(),
                        KeyCode::Esc => self.stop_editing(),
                        _ => {
                            self.input.handle_event(&event);
                        }
                    },
                }
            }
        }
    }

    fn new_round(&mut self) {
        self.game.new_round();
        self.feedback = None;
        self.input = Input::default();
        self.game_state = GameState::Answering;
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

    fn render_feedback(&self, frame: &mut Frame, area: Rect) {
        let style = match self.feedback {
            Some(Feedback::Correct) => Style::default().fg(Color::Green),
            Some(Feedback::Incorrect(_)) => Style::default().fg(Color::LightRed),
            None => Style::default(),
        };
        let Some(feedback) = &self.feedback else {
            return;
        };

        let text = match feedback {
            Feedback::Correct => "Correct",
            Feedback::Incorrect(_) => "Incorrect",
        };

        let feedback_area = Paragraph::new(text)
            .alignment(Alignment::Center)
            .style(style);

        frame.render_widget(feedback_area, area);
    }

    fn render(&self, frame: &mut Frame) {
        let [_, centered_area, _] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Min(35),
            Constraint::Fill(1),
        ])
        .areas(frame.area());

        let [_, target_area, messages_area, input_area, feedback_area, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(centered_area);

        self.render_target_conjugation(frame, target_area);
        self.render_vocab(frame, messages_area);
        self.render_feedback(frame, feedback_area);
        self.render_input(frame, input_area);
    }

    fn render_input(&self, frame: &mut Frame, area: Rect) {
        let width = area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);
        let input = Paragraph::new(self.input.value())
            .scroll((0, scroll as u16))
            .block(Block::bordered().border_type(ratatui::widgets::BorderType::Rounded));
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

        let vocab = Text::from(Line::from(vec![
            Span::from(&self.game.round.word_to_conjugate.base),
            Span::from(format!("({})", &self.game.round.word_to_conjugate.furigana)),
            Span::styled(&self.game.round.word_to_conjugate.ender, style),
        ]));

        let vocab_area = Paragraph::new(vocab)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title(" Conjugate "),
            );

        frame.render_widget(vocab_area, area);
    }

    fn render_target_conjugation(&self, frame: &mut Frame, area: Rect) {
        let target_area = Paragraph::new(self.game.round.format_conjugations())
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(target_area, area);
    }
}
