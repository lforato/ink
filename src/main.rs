use std::io::{self, Result};

use flexi_logger::{FileSpec, Logger, detailed_format};
use ink::{
    page::Page,
    widgets::chat_mode::{ChatMode, Mode},
};
use log::info;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Rect,
    widgets::Widget,
};

fn main() -> io::Result<()> {
    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(FileSpec::default().directory("logs").suppress_timestamp())
        .format(detailed_format)
        .start()
        .unwrap();

    let mut term = ratatui::init();
    let app_result = App::default().run(&mut term);
    ratatui::restore();
    app_result
}

#[derive(Default, Debug)]
struct App {
    pub exit: bool,
    pub page: Page,
    pub mode: Mode,
}

impl App {
    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode
    }

    fn exit(&mut self) {
        self.exit = true
    }

    fn run(&mut self, term: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            term.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('j') => {
                if let Page::Modes = self.page {
                    self.set_mode(self.mode.next());
                }
            }
            KeyCode::Char('k') => {
                if let Page::Modes = self.page {
                    self.set_mode(self.mode.prev());
                }
            }
            KeyCode::Enter => match self.mode {
                Mode::Default => self.page = Page::DefaultChat,
                Mode::Debug => self.page = Page::DebugChat,
            },
            _ => {}
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_chat_modes(area, buf);
    }
}

impl App {
    fn render_chat_modes(&self, area: Rect, buf: &mut Buffer) {
        let mut chat_mode = ChatMode::new();
        chat_mode.set_mode(&self.mode);
        chat_mode.render(area, buf);
    }
}
