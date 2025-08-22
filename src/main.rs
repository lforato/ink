use flexi_logger::{FileSpec, Logger, detailed_format};
use ink::widgets::message::Message;
use log::info;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::Rect,
    widgets::Widget,
};
use std::io::{self, Result};

fn main() -> io::Result<()> {
    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(FileSpec::default().directory("logs"))
        .format(detailed_format)
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed: {}", e));

    info!("Started logger");

    let mut term = ratatui::init();
    let app_result = App::default().run(&mut term);
    ratatui::restore();
    app_result
}

#[derive(Default, Debug)]
struct App {
    pub exit: bool,
    pub msg: Message,
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
                    KeyCode::Char('l') => self.msg.increase_h_scroll_state(),
                    KeyCode::Char('h') => self.msg.decrease_h_scroll_state(),
                    KeyCode::Char('j') => self.msg.increase_v_scroll_state(),
                    KeyCode::Char('k') => self.msg.decrease_v_scroll_state(),
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
