use iced::{
    widget::{column, container, text},
    Element, Fill,
};

use crate::app::{Message, State};

pub fn view(state: &State) -> Element<'_, Message> {
    let list = state.bindings.iter().take(14).fold(
        column![
            text("Shortcut library").size(32),
            text(format!("{} shortcuts loaded", state.bindings.len())).size(16)
        ]
        .spacing(14),
        |column, binding| {
            column.push(
                container(
                    column![
                        text(&binding.action).size(17),
                        text(binding.hotkey.to_string()).size(14),
                        text(format!("from {}", binding.source)).size(11)
                    ]
                    .spacing(4),
                )
                .padding(12)
                .width(Fill)
                .style(container::bordered_box),
            )
        },
    );
    list.into()
}
