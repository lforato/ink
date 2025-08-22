use std::io::{self, Result};

use flexi_logger::{FileSpec, Logger, detailed_format};
use ink::widgets::message::Message;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::Rect,
    widgets::{ListState, Widget},
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
    pub list_state: ListState,
    pub msg: Message
}

impl App {
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

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char('q') => self.exit(),
                    _ => {}
                }
            }
            _ => {}
        }

        Ok(())
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.msg.render(area, buf);
    }
}
