use std::time::Instant;

use iced::Task;

use crate::{
    input::Hotkey,
    learning::{score_attempt, select_next, SessionFeedback},
    omarchy::load_bindings,
};

use super::{Message, State};

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Navigate(page) => state.page = page,
        Message::KeyPressed(key, modifiers) => {
            if state.feedback.is_some() || state.current_binding().is_none() {
                return Task::none();
            }
            let Some(pressed) = Hotkey::from_iced(key, modifiers) else {
                return Task::none();
            };
            let expected = state
                .current_binding()
                .map(|binding| binding.hotkey.clone());
            if let Some(expected) = expected {
                let result = score_attempt(
                    &expected,
                    &pressed,
                    state.challenge_started.elapsed(),
                    state.progress.streak,
                );
                state.progress.record(&expected, &result);
                state.feedback = Some(SessionFeedback::from(result));
                state.save();
            }
        }
        Message::NextChallenge => {
            state.current = select_next(&state.bindings, &state.progress, state.current);
            state.feedback = None;
            state.challenge_started = Instant::now();
        }
        Message::ReloadBindings => match load_bindings(&state.config_path) {
            Ok(bindings) if !bindings.is_empty() => {
                state.bindings = bindings;
                state.current = select_next(&state.bindings, &state.progress, None);
                state.feedback = None;
                state.error = None;
            }
            Ok(_) => state.error = Some("No Hyprland bindings were found".into()),
            Err(error) => state.error = Some(error.to_string()),
        },
        Message::ResetProgress => {
            state.progress = Default::default();
            state.feedback = None;
            state.save();
        }
    }
    Task::none()
}
