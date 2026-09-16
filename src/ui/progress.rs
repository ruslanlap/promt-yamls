use iced::{
    widget::{column, container, progress_bar, row, text},
    Element, Fill,
};

use crate::app::{Message, State};

pub fn view(state: &State) -> Element<'_, Message> {
    let accuracy = state.progress.accuracy();
    column![
        text("Your progress").size(32),
        row![
            metric("Accuracy", format!("{:.0}%", accuracy * 100.0)),
            metric("Total XP", state.progress.xp.to_string()),
            metric("Best streak", state.progress.best_streak.to_string()),
        ]
        .spacing(16),
        text(format!(
            "{} correct of {} attempts",
            state.progress.correct_attempts, state.progress.total_attempts
        )),
        progress_bar(0.0..=1.0, accuracy).height(12),
    ]
    .spacing(24)
    .into()
}

pub fn weak_keys(state: &State) -> Element<'_, Message> {
    let mut stats: Vec<_> = state.progress.bindings.iter().collect();
    stats.sort_by(|(_, left), (_, right)| {
        let left_rate = left.correct as f32 / left.attempts.max(1) as f32;
        let right_rate = right.correct as f32 / right.attempts.max(1) as f32;
        left_rate.total_cmp(&right_rate)
    });
    stats
        .into_iter()
        .take(12)
        .fold(
            column![
                text("Weak keys").size(32),
                text("Shortcuts with the lowest accuracy appear first.")
            ]
            .spacing(14),
            |column, (key, stat)| {
                column.push(
                    container(row![
                        text(key).width(Fill),
                        text(format!("{} / {} correct", stat.correct, stat.attempts)),
                    ])
                    .padding(14)
                    .width(Fill)
                    .style(container::bordered_box),
                )
            },
        )
        .into()
}

fn metric<'a>(label: &'a str, value: String) -> Element<'a, Message> {
    container(column![text(value).size(30), text(label).size(14)].spacing(4))
        .padding(20)
        .width(Fill)
        .style(container::bordered_box)
        .into()
}
