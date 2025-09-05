use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
};
use flexi_logger::{FileSpec, Logger, detailed_format};
use ink::widgets::chat::Chat;
use log::info;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseEventKind},
    layout::Rect,
    widgets::Widget,
};
use serde::{Deserialize, Serialize};
use std::{
    io::{Result as Resultt, stdout},
    time::Duration,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(FileSpec::default().directory("logs"))
        .format(detailed_format)
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed: {}", e));

    info!("Started logger");

    let mut stdout = stdout();
    execute!(stdout, EnableMouseCapture)?;

    let mut term = ratatui::init();
    let app_result = App::new().run(&mut term);
    ratatui::restore();

    execute!(stdout, DisableMouseCapture)?;
    app_result
}

#[derive(Debug)]
struct App<'a> {
    pub exit: bool,
    pub chat: Chat<'a>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Chunk {
    model: String,
    created_at: String,
    done: bool,
    message: Msg,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Msg {
    role: String,
    content: String,
}

impl<'a> App<'a> {
    fn new() -> Self {
        let chat = Chat::new(Vec::new());
        Self { chat, exit: false }
    }

    fn exit(&mut self) {
        self.exit = true
    }

    fn run(&mut self, term: &mut DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
        while !self.exit {
            term.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Resultt<()> {
        if let Ok(has_event) = event::poll(Duration::from_millis(100)) {
            if !has_event {
                return Ok(());
            }
        }

        if !self.chat.textarea.is_selected {
            match event::read()? {
                Event::Key(key_event) => {
                    if key_event.kind == KeyEventKind::Press {
                        match key_event.code {
                            KeyCode::Char('q') => self.exit(),
                            KeyCode::Char('j') => self.chat.scroll_down(),
                            KeyCode::Char('k') => self.chat.scroll_up(),
                            KeyCode::Tab => self.chat.select_next(),
                            KeyCode::BackTab => self.chat.select_prev(),
                            _ => {}
                        }
                    }
                }

                Event::Mouse(mouse_event) => match mouse_event.kind {
                    MouseEventKind::ScrollUp => self.chat.scroll_up(),
                    MouseEventKind::ScrollDown => self.chat.scroll_down(),
                    _ => {}
                },
                _ => {}
            }
        }

        Ok(())
    }
}

impl<'a> Widget for &mut App<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.chat.render(area, buf);
    }
}
