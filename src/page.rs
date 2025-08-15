#[derive(Debug)]
pub enum Page {
    Modes,
    DebugChat,
    DefaultChat
}

impl Default for Page {
    fn default() -> Self {
        Self::Modes

    }
}
