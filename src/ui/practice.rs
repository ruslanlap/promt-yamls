use iced::{
    alignment,
    widget::{button, column, container, horizontal_rule, text},
    Element, Fill,
};

use crate::app::{Message, State};

pub fn view(state: &State) -> Element<'_, Message> {
    let content: Element<'_, Message> = if let Some(binding) = state.current_binding() {
        let status: Element<'_, Message> = if let Some(feedback) = &state.feedback {
            let marker = if feedback.correct { "✓" } else { "×" };
            column![
                text(format!("{marker} {}", feedback.title)).size(30),
                text(&feedback.detail).size(16),
                button("Next challenge →")
                    .padding([12, 22])
                    .on_press(Message::NextChallenge),
            ]
            .align_x(alignment::Horizontal::Center)
            .spacing(12)
            .into()
        } else {
            column![
                text("waiting for shortcut…").size(17),
                text("Keep this window focused and press the key combination.").size(14),
            ]
            .align_x(alignment::Horizontal::Center)
            .spacing(8)
            .into()
        };
        column![
            text("PRACTICE").size(14),
            text(&binding.action).size(34),
            text("Press the matching hotkey").size(17),
            horizontal_rule(1),
            status,
        ]
        .align_x(alignment::Horizontal::Center)
        .spacing(24)
        .into()
    } else {
        column![
            text("No shortcuts available").size(30),
            text(
                state
                    .error
                    .as_deref()
                    .unwrap_or("Choose a Hyprland configuration in Settings.")
            ),
            button("Reload configuration").on_press(Message::ReloadBindings),
        ]
        .align_x(alignment::Horizontal::Center)
        .spacing(16)
        .into()
    };

    container(content).center_x(Fill).center_y(Fill).into()
}
