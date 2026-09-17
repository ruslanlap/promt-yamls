mod home;
mod practice;
mod progress;
mod settings;

use iced::{
    widget::{button, column, container, row, text, vertical_space},
    Element, Fill,
};

use crate::app::{Message, Page, State};

pub fn view(state: &State) -> Element<'_, Message> {
    let navigation = [
        (Page::Learn, "Learn"),
        (Page::Practice, "Practice"),
        (Page::WeakKeys, "Weak keys"),
        (Page::Progress, "Progress"),
        (Page::Settings, "Settings"),
    ]
    .into_iter()
    .fold(
        column![text("KEYARCHY").size(24)].spacing(12),
        |menu, (page, label)| {
            let mut item = button(text(label).size(16)).width(Fill).padding(12);
            if state.page != page {
                item = item.on_press(Message::Navigate(page));
            }
            menu.push(item)
        },
    );

    let sidebar = container(
        column![
            navigation,
            vertical_space(),
            text(format!("🔥 {} streak", state.progress.streak)),
            text(format!("{} XP", state.progress.xp)).size(14),
        ]
        .spacing(8),
    )
    .width(220)
    .height(Fill)
    .padding(24);

    let content: Element<'_, Message> = match state.page {
        Page::Learn => home::view(state),
        Page::Practice => practice::view(state),
        Page::WeakKeys => progress::weak_keys(state),
        Page::Progress => progress::view(state),
        Page::Settings => settings::view(state),
    };

    row![
        sidebar,
        container(content).padding(40).width(Fill).height(Fill)
    ]
    .into()
}
