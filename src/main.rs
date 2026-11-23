mod package;
mod parser;

use crate::package::Package;
use crate::parser::parse_packages;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    symbols::{self, border},
    text::{Line, Text},
    widgets::{Block, Paragraph, Tabs, Widget},
};
use std::io;
use std::process::Command;
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
    search_query: Input,
    input_mode: InputMode,
    results: Vec<Package>,
    error: Option<String>,
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        let search_bar = Paragraph::new(self.search_query.value()).block(
            Block::bordered()
                .title(" Search ")
                .border_type(ratatui::widgets::BorderType::Rounded),
        );

        search_bar.render(chunks[0], buf);
        let results_text = if let Some(error) = &self.error {
            Text::from(error.as_str())
        } else {
            Text::from(
                self.results
                    .iter()
                    .map(|package| {
                        Line::from(vec![
                            package.name.clone().bold(),
                            format!("  v{}", package.version).yellow(),
                        ])
                    })
                    .collect::<Vec<Line>>(),
            )
        };

        Paragraph::new(results_text)
            .block(Block::bordered().title(" Search Results "))
            .render(chunks[1], buf);
        Paragraph::new(self.search_query.value())
            .block(Block::bordered().title(" Search NixOS Packages "))
            .render(chunks[0], buf);
    }
}

impl App {
    /// runs the application's main loop until the user quits
    fn search_packages(&mut self) {
        let query = self.search_query.value().trim().to_owned();

        if query.is_empty() {
            return;
        }

        self.results.clear();
        self.error = None;

        let output = Command::new("nix-search")
            .arg("--json")
            .arg("--search")
            .arg(&query)
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);

                    match parse_packages(&stdout) {
                        Ok(packages) => {
                            self.results = packages;
                        }

                        Err(error) => {
                            self.error = Some(error);
                        }
                    }
                } else {
                    self.error = Some(String::from_utf8_lossy(&output.stderr).to_string());
                }
            }

            Err(error) => {
                self.error = Some(error.to_string());
            }
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    fn exit(&mut self) {
        self.exit = true;
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
        if self.input_mode == InputMode::Editing {
            let area = frame.area();

            let x = area.x + 1 + self.search_query.visual_cursor() as u16;
            let y = area.y + 1;

            frame.set_cursor_position((x, y));
        }
    }
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.input_mode {
            InputMode::Normal => match key_event.code {
                KeyCode::Char('q') => self.exit(),

                KeyCode::Char('i') => {
                    self.input_mode = InputMode::Editing;
                }

                _ => {}
            },
            InputMode::Editing => match key_event.code {
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                }

                KeyCode::Enter => {
                    self.search_packages();
                    self.input_mode = InputMode::Normal;
                }

                _ => {
                    self.search_query.handle_event(&Event::Key(key_event));
                }
            },
        }
    }
}
