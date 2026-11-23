mod package;
mod parser;

use crate::package::Package;
use crate::parser::parse_packages;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{
        Block, BorderType, List, ListItem, ListState, Paragraph, StatefulWidget, Widget,
    },
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
    exit: bool,
    search_query: Input,
    input_mode: InputMode,
    results: Vec<Package>,
    error: Option<String>,
    selected: usize,
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let editing = self.input_mode == InputMode::Editing;
        let focused_border = Style::new().cyan();
        let unfocused_border = Style::new();

        let search_bar = Paragraph::new(self.search_query.value()).block(
            Block::bordered()
                .title(" Search NixOS Packages ")
                .border_type(BorderType::Rounded)
                .border_style(if editing {
                    focused_border
                } else {
                    unfocused_border
                }),
        );
        search_bar.render(chunks[0], buf);

        let results_block = Block::bordered()
            .title(" Search Results ")
            .border_type(BorderType::Rounded)
            .border_style(if editing {
                unfocused_border
            } else {
                focused_border
            });

        let mut results_state = ListState::default();
        let items = if let Some(error) = &self.error {
            vec![ListItem::new(Text::from(error.as_str()).style(Style::new().red()))]
        } else if self.results.is_empty() {
            vec![ListItem::new(Text::from(
                "No results yet. Press <i> to search for a package.",
            ))]
        } else {
            results_state.select(Some(self.selected));
            self.results.iter().map(package_item).collect()
        };

        let results_list = List::new(items)
            .block(results_block)
            .highlight_style(Style::new().bg(Color::Cyan).fg(Color::Black))
            .highlight_symbol(">> ");
        StatefulWidget::render(&results_list, chunks[1], buf, &mut results_state);

        let instructions = Line::from(vec![
            " Edit ".into(),
            "<i>".cyan().bold(),
            " Search ".into(),
            "<Enter>".cyan().bold(),
            " Cancel ".into(),
            "<Esc>".cyan().bold(),
            " Navigate ".into(),
            "<Up/Down>".cyan().bold(),
            " Quit ".into(),
            "<q>".cyan().bold(),
        ]);
        Paragraph::new(instructions)
            .block(
                Block::bordered()
                    .title(" Instructions ")
                    .border_type(BorderType::Rounded),
            )
            .render(chunks[2], buf);
    }
}

fn package_item(package: &Package) -> ListItem<'_> {
    let mut lines = vec![Line::from(vec![
        package.name.clone().bold(),
        format!(" v{}", package.version).yellow(),
        format!("  {}.{}", package.attr_set, package.attr_name).dim(),
    ])];

    if !package.description.is_empty() {
        lines.push(Line::from(vec![
            "    ".into(),
            package.description.clone().dim(),
        ]));
    }

    let mut links = Vec::new();
    if !package.homepage.is_empty() {
        links.push(Line::from(vec![
            "homepage: ".into(),
            package.homepage.join(", ").blue(),
        ]));
    }
    if !package.platforms.is_empty() {
        links.push(Line::from(vec![
            "platforms: ".into(),
            package.platforms.join(", ").dark_gray(),
        ]));
    }
    if !links.is_empty() {
        lines.push(detail_line(links));
    }

    let mut metadata = Vec::new();
    if !package.programs.is_empty() {
        metadata.push(Line::from(vec![
            "programs: ".into(),
            package.programs.join(", ").cyan(),
        ]));
    }
    if !package.outputs.is_empty() {
        metadata.push(Line::from(vec![
            "outputs: ".into(),
            package.outputs.join(", ").yellow(),
        ]));
    }
    if !package.position.is_empty() {
        metadata.push(Line::from(vec![
            "position: ".into(),
            package.position.clone().dark_gray(),
        ]));
    }
    if !metadata.is_empty() {
        lines.push(detail_line(metadata));
    }

    ListItem::new(lines)
}

fn detail_line<'a>(fields: Vec<Line<'a>>) -> Line<'a> {
    let mut spans = vec!["    ".into()];
    for (index, field) in fields.into_iter().enumerate() {
        if index > 0 {
            spans.push("  |  ".into());
        }
        spans.extend(field.spans);
    }
    Line::from(spans)
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
        self.selected = 0;

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
                KeyCode::Char('i') => self.input_mode = InputMode::Editing,
                KeyCode::Up => {
                    self.selected = self.selected.saturating_sub(1);
                }
                KeyCode::Down => {
                    if !self.results.is_empty() {
                        self.selected = (self.selected + 1).min(self.results.len() - 1);
                    }
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
