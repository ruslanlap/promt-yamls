use std::fmt;

use iced::keyboard::{key::Named, Key, Modifiers};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hotkey {
    pub super_key: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub key: String,
}

impl Hotkey {
    pub fn parse(modifiers: &str, key: &str) -> Self {
        let modifiers = modifiers.to_ascii_uppercase();
        Self {
            super_key: modifiers.contains("SUPER") || modifiers.contains("META"),
            ctrl: modifiers.contains("CTRL") || modifiers.contains("CONTROL"),
            alt: modifiers.contains("ALT") || modifiers.contains("MOD1"),
            shift: modifiers.contains("SHIFT"),
            key: normalize_key(key),
        }
    }

    pub fn from_iced(key: Key, modifiers: Modifiers) -> Option<Self> {
        let key = match key.as_ref() {
            Key::Character(value) => normalize_key(value),
            Key::Named(named) => {
                let debug_name = format!("{named:?}");
                if matches!(debug_name.as_str(), "Super" | "Control" | "Alt" | "Shift") {
                    return None;
                }
                named_key(named)
            }
            Key::Unidentified => "unknown".into(),
        };
        Some(Self {
            super_key: modifiers.logo(),
            ctrl: modifiers.control(),
            alt: modifiers.alt(),
            shift: modifiers.shift(),
            key,
        })
    }
}

impl fmt::Display for Hotkey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();
        if self.super_key {
            parts.push("Super");
        }
        if self.ctrl {
            parts.push("Ctrl");
        }
        if self.alt {
            parts.push("Alt");
        }
        if self.shift {
            parts.push("Shift");
        }
        parts.push(&self.key);
        write!(formatter, "{}", parts.join(" + "))
    }
}

fn normalize_key(key: &str) -> String {
    match key.trim().trim_matches('$').to_ascii_lowercase().as_str() {
        "return" => "Enter".into(),
        "escape" => "Esc".into(),
        "space" => "Space".into(),
        value => value.to_ascii_uppercase(),
    }
}

fn named_key(key: Named) -> String {
    match key {
        Named::Enter => "Enter".into(),
        Named::Escape => "Esc".into(),
        Named::Space => "Space".into(),
        Named::Tab => "Tab".into(),
        Named::ArrowUp => "UP".into(),
        Named::ArrowDown => "DOWN".into(),
        Named::ArrowLeft => "LEFT".into(),
        Named::ArrowRight => "RIGHT".into(),
        Named::Backspace => "Backspace".into(),
        other => format!("{other:?}").to_ascii_uppercase(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_hyprland_modifiers() {
        let hotkey = Hotkey::parse("$mainMod SHIFT", "3");
        assert!(hotkey.super_key);
        assert!(hotkey.shift);
        assert_eq!(hotkey.to_string(), "Super + Shift + 3");
    }
}
