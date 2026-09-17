use iced::{
    widget::{button, column, container, text},
    Element, Fill,
};

use crate::app::{Message, State};

pub fn view(state: &State) -> Element<'_, Message> {
    let error = state
        .error
        .as_deref()
        .unwrap_or("Configuration loaded successfully.");
    column![
        text("Settings").size(32),
        text("Hyprland configuration").size(18),
        container(text(state.config_path.display().to_string()))
            .padding(14)
            .width(Fill)
            .style(container::bordered_box),
        text(error).size(14),
        button("Reload bindings")
            .padding([10, 18])
            .on_press(Message::ReloadBindings),
        text("Learning data").size(18),
        text("Progress is stored locally in your XDG data directory."),
        button("Reset progress")
            .padding([10, 18])
            .on_press(Message::ResetProgress),
    ]
    .spacing(16)
    .into()
}
