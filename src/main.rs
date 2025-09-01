use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
};
use flexi_logger::{detailed_format, FileSpec, Logger};
use ink::widgets::chat::Chat;
use log::info;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseEventKind},
    layout::Rect,
    widgets::Widget,
    DefaultTerminal, Frame,
};
use std::{
    io::{self, stdout, Result},
    time::Duration,
};

fn main() -> io::Result<()> {
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

impl<'a> App<'a> {
    fn new() -> Self {
        let mut messages = Vec::new();
        let msg = String::from("0: hello worlddd\n\n\n\n\n");
        messages.push(msg);

        let msg = String::from("1: hello world\n\n\n\n\n\n\n\n");
        messages.push(msg);

        let msg = String::from("2: hello world\n\n\n\n");
        messages.push(msg);

        let msg = String::from("3: hello world\n\n\n\n\n\n");
        messages.push(msg);

        let msg = String::from("4: hello world\n\n\n\n\n\n");
        messages.push(msg);

        let msg = String::from("5: hello world\n\n\n\n\n\n\n");
        messages.push(msg);

        let msg = String::from("6: hello world\n\n\n\n\n\n\n\n\n\n\n\n");
        messages.push(msg);

        let msg = String::from("7: hello world\n\n\n\n");
        messages.push(msg);

        let msg = String::from("8: hello world");
        messages.push(msg);

        let msg = String::from("9: hello world");
        messages.push(msg);

        let msg = String::from("10: hello world");
        messages.push(msg);

        let msg = String::from("11: hello world");
        messages.push(msg);

        let msg = String::from("12: hello world");
        messages.push(msg);

        let msg = String::from("13: hello world");
        messages.push(msg);

        let msg = String::from("14: hello world");
        messages.push(msg);

        let chat = Chat::new(messages);

        Self { chat, exit: false }
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

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Ok(has_event) = event::poll(Duration::from_millis(100)) {
            if !has_event {
                return Ok(());
            }
        }

        if self.chat.textarea.is_selected {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Esc {
                    self.chat.textarea.is_selected = false
                }
                self.chat.textarea.area.input(key);
            }
        } else {
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
