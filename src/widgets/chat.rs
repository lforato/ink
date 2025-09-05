use crate::widgets::{
    message::{Message, OFFSET, Role},
    textarea::TextArea,
};
use futures_util::stream::StreamExt;
use log::info;
use ratatui::crossterm::event::{self, Event, KeyCode};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget},
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    io::Result,
    sync::{Arc, Mutex, mpsc},
    time::Duration,
};
use tokio::task;

#[derive(Debug)]
pub struct Chat<'a> {
    pub selected_message_id: usize,
    pub messages: Vec<Message>,
    pub textarea: TextArea<'a>,
    /// the total height of the chat screen including the entire chat history
    pub height: usize,
    /// used to render the scrollbar, represents where
    /// in the scroll_area that the scrollbar is located
    pub scroll_state: usize,
    /// used to render the scrollbar, it represents
    /// how much space the scrollbar will have for scrolling
    pub scroll_area: usize,
    pub rx: mpsc::Receiver<Chunk>,
    pub tx: mpsc::Sender<Chunk>,
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

#[derive(Deserialize, Serialize, Debug)]
struct OllamaReqBody {
    model: String,
    messages: Vec<Msg>,
}

impl Default for OllamaReqBody {
    fn default() -> Self {
        Self {
            model: String::from("deepseek-r1:8b"),
            messages: Vec::new(),
        }
    }
}

pub const MARGIN: i32 = 1;

impl<'a> Chat<'a> {
    pub fn new(input: Vec<(String, Role)>) -> Self {
        let mut messages: Vec<Message> = input
            .into_iter()
            .enumerate()
            .map(|(i, item)| Message::new(i, item.0, false, item.1))
            .collect();

        if messages.len() > 0 {
            messages[0].is_selected = true;
        }

        let textarea = TextArea::default();
        let (tx, rx) = mpsc::channel::<Chunk>();

        Self {
            rx,
            tx,
            messages,
            textarea,
            height: 0,
            scroll_area: 0,
            scroll_state: 0,
            selected_message_id: 0,
        }
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Ok(has_event) = event::poll(Duration::from_millis(100)) {
            if !has_event {
                return Ok(());
            }
        }

        if self.textarea.is_selected {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Esc {
                    self.textarea.is_selected = false
                }
                if key.code == KeyCode::Enter {
                    self.textarea.area.select_all();
                    self.textarea.area.cut();
                    self.push_user_message(self.textarea.area.yank_text());
                    self.textarea.area.set_yank_text("");
                    self.start_generating();
                }
                self.textarea.area.input(key);
            }
        }

        Ok(())
    }

    pub fn scroll_up(&mut self) {
        if self.scroll_state == 0 {
            return;
        }
        self.scroll_state -= 1;
    }

    pub fn scroll_down(&mut self) {
        if self.scroll_state > self.scroll_area {
            return;
        }
        self.scroll_state += 1;
    }

    pub fn select_next(&mut self) {
        if self.messages.is_empty() {
            return;
        }

        if self.selected_message_id + 1 >= self.messages.len() {
            return;
        }

        self.messages[self.selected_message_id].is_selected = false;
        self.selected_message_id += 1;
        self.messages[self.selected_message_id].is_selected = true;
    }

    pub fn select_prev(&mut self) {
        if self.messages.is_empty() || self.selected_message_id == 0 {
            return;
        }

        self.messages[self.selected_message_id].is_selected = false;
        self.selected_message_id -= 1;
        self.messages[self.selected_message_id].is_selected = true;
    }

    pub fn set_scroll_area(&mut self, scroll_area: usize) -> () {
        self.scroll_area = scroll_area;
    }

    pub fn push_user_message(&mut self, value: String) -> usize {
        let idx = self.messages.len();
        let msg = Message::new(idx, value, false, Role::User);
        self.messages.push(msg);
        idx
    }

    pub fn render_vertical_scrollbar(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        viewport_height: usize,
    ) {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"))
            .track_symbol(Some("│"));

        let scrollable_height = self.height.saturating_sub(viewport_height);
        self.set_scroll_area(scrollable_height);

        let mut scrollbar_state =
            ScrollbarState::new(scrollable_height).position(self.scroll_state);

        StatefulWidget::render(scrollbar, area, buf, &mut scrollbar_state);
    }

    fn generate_req_body(&self) -> OllamaReqBody {
        let mut response = OllamaReqBody::default();

        self.messages.iter().for_each(|item| {
            let msg = Msg {
                role: item.role.to_lower_string(),
                content: item.text.clone(),
            };
            response.messages.push(msg);
        });

        response
    }

    /// This method is going to spawn an async thread and it will update the response String
    /// with the latest response, once it is done, it calls a method that will lock the response
    /// of the LLM
    pub fn start_generating(&mut self) {
        let body = self.generate_req_body();
        let tx = self.tx.clone();

        self.messages.push(Message::new(
            self.messages.len(),
            String::from(""),
            true,
            Role::System,
        ));

        task::spawn(async move {
            let json = serde_json::to_string(&body).unwrap();

            let client = Client::new();
            let bytes = client
                .post("http://localhost:11434/api/chat")
                .body(json)
                .header("Content-Type", "application/json")
                .send()
                .await
                .unwrap();

            let mut stream = bytes.bytes_stream();

            while let Some(chunk) = stream.next().await {
                if let Ok(chunk) = chunk {
                    let text = String::from_utf8_lossy(chunk.as_ref()).to_string();
                    if let Ok(parsed) = serde_json::from_str::<Chunk>(&text) {
                        tx.send(parsed).unwrap();
                    }
                }
            }
        });
    }
}

impl<'a> Widget for &mut Chat<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Ok(chan_msg) = self.rx.try_recv() {
            if let Some(last_msg) = self.messages.last_mut() {
                last_msg.text.push_str(chan_msg.message.content.as_ref());
                info!("{:?}", last_msg.text);
            }
        }

        self.handle_events().unwrap();

        let layout = Layout::horizontal([
            Constraint::Percentage(5),
            Constraint::Percentage(90),
            Constraint::Max(2),
        ])
        .split(area);

        Block::bordered().render(layout[1], buf);
        let chat_inner = Block::bordered().inner(layout[1]);

        let chat_inner_layout =
            Layout::vertical([Constraint::Percentage(100), Constraint::Length(10)])
                .split(chat_inner);

        let chat_inner = chat_inner_layout[0];
        let chat_textarea = chat_inner_layout[1];

        let total_height: u16 = self.messages.iter().map(|m| m.height).sum();
        self.height = total_height as usize + OFFSET;

        let scroll_top = self.scroll_state as i32;
        let visible_height = chat_inner.height as i32;

        let mut y = chat_inner.y as i32;
        let mut new_id = self.selected_message_id;
        let len = self.messages.len();

        self.textarea.render(chat_textarea, buf);

        for item in self.messages.iter_mut() {
            let h = item.height as i32;

            let msg_top = y - chat_inner.y as i32;
            let msg_bottom = msg_top + h;

            // if we could see the entire page, what line is the last one that is visible at the
            // moment?
            // ex:
            // 1
            // 2 | top
            // 3 | bottom
            // 4
            // 5
            // we can say in the example that visible_bottom is 3 because it is the last line
            // visible, and we can say that 2 is the visible_top for the same reason

            let visible_top = scroll_top.max(msg_top);
            let visible_bottom = (scroll_top + visible_height) as i32;

            // this is the height that is still on screen
            let clip_height = (msg_bottom.min(visible_bottom) - visible_top).max(0);
            let clip_start = (visible_top - msg_top).max(0);

            if clip_height > 0 {
                let rect = Rect {
                    x: chat_inner.x,
                    y: (chat_inner.y as i32 + visible_top - scroll_top as i32) as u16,
                    width: chat_inner.width,
                    height: clip_height as u16,
                };

                item.set_skip_lines(clip_start as u16);

                item.render(rect, buf);
            }

            let msg_top_hit_bottom = msg_top + MARGIN == visible_bottom;
            if msg_top_hit_bottom && item.is_selected && item.id > 0 {
                new_id -= 1;
                item.is_selected = false;
            }

            let msg_bottom_hit_top = msg_bottom == visible_top + MARGIN;
            if msg_bottom_hit_top && item.is_selected {
                if item.id + 1 < len {
                    new_id += 1;
                    item.is_selected = false;
                }
            }

            y += h;
        }

        self.selected_message_id = new_id;
        if !self.messages.is_empty() {
            self.messages[new_id].is_selected = true;
        }

        self.render_vertical_scrollbar(layout[2], buf, chat_inner.height as usize);
    }
}
