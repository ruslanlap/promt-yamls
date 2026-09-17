mod messages;
mod state;
mod update;

pub use messages::{Message, Page};
pub use state::State;
pub use update::update;

use iced::{event, keyboard, Subscription, Theme};

pub fn subscription(_state: &State) -> Subscription<Message> {
    event::listen_with(|event, status, _window| match event {
        iced::Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. })
            if status == event::Status::Ignored =>
        {
            Some(Message::KeyPressed(key, modifiers))
        }
        _ => None,
    })
}

pub fn theme(_state: &State) -> Theme {
    Theme::TokyoNight
}
