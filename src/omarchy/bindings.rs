use crate::input::Hotkey;

#[derive(Debug, Clone)]
pub struct Binding {
    pub hotkey: Hotkey,
    pub action: String,
    pub source: String,
}
