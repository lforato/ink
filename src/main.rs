use crossterm::execute;
use ratatui::crossterm::event::{
    self, Event, KeyCode, KeyEventKind, MouseButton, MouseEvent, MouseEventKind,
};

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use flexi_logger::{FileSpec, Logger, detailed_format};
use ink::widgets::chat::Chat;
use log::info;
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::Rect, widgets::Widget};
use serde::{Deserialize, Serialize};
use std::{
    io::{self, stdout},
    time::Duration,
};
use uuid::Uuid;

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
    pub selected_id: Option<Uuid>,
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
        Self {
            chat,
            exit: false,
            selected_id: None,
        }
    }

    fn exit(&mut self) {
        self.exit = true
    }

    fn run(&mut self, term: &mut DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
        while !self.exit {
            term.draw(|frame| self.draw(frame))?;
            self.define_selected_item();
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn define_selected_item(&mut self) {
        self.chat.textarea.is_selected = false;
        self.chat
            .messages
            .iter_mut()
            .for_each(|f| f.is_selected = false);

        if let None = self.selected_id {
            return;
        }

        if self.chat.textarea.id == self.selected_id.unwrap() {
            self.chat.textarea.is_selected = true;
        }

        self.chat.messages.iter_mut().for_each(|item| {
            if item.id == self.selected_id.unwrap() {
                item.is_selected = true
            }
        });
    }

    fn handle_mouse_click_events(&mut self, mouse_event: MouseEvent) {
        let x = mouse_event.column;
        let y = mouse_event.row;

        if self.chat.textarea.is_within(x, y) {
            self.selected_id = Some(self.chat.textarea.id);
        }

        self.chat.messages.iter_mut().for_each(|item| {
            if item.is_within(x, y) {
                self.selected_id = Some(item.id);
            }
        });
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Ok(has_event) = event::poll(Duration::from_millis(100)) {
            if !has_event {
                return Ok(());
            }
        }

        let event = event::read().unwrap();

        if let Event::Key(key_event) = event {
            if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Esc {
                self.selected_id = None
            }
        }

        if let Event::Mouse(mouse_event) = event {
            match mouse_event.kind {
                MouseEventKind::ScrollUp => self.chat.scroll_up(),
                MouseEventKind::ScrollDown => self.chat.scroll_down(),
                MouseEventKind::Down(btn) => {
                    if btn == MouseButton::Left {
                        self.handle_mouse_click_events(mouse_event)
                    }
                }
                _ => {}
            }
        }

        if self.chat.textarea.is_selected {
            return self.chat.handle_events(event);
        }

        self.chat.messages.iter_mut().for_each(|f| {
            if f.is_selected {
                f.handle_events(event.clone()).unwrap();
            }
        });

        if let Event::Key(key_event) = event {
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

        Ok(())
    }
}

impl<'a> Widget for &mut App<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.chat.render(area, buf);
    }
}
