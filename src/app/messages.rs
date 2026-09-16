use iced::keyboard;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Learn,
    Practice,
    WeakKeys,
    Progress,
    Settings,
}

#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Page),
    KeyPressed(keyboard::Key, keyboard::Modifiers),
    NextChallenge,
    ReloadBindings,
    ResetProgress,
}
