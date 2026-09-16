use std::{path::PathBuf, time::Instant};

use iced::Task;

use crate::{
    learning::{select_next, SessionFeedback},
    omarchy::{load_bindings, Binding},
    storage::{Progress, ProgressStore},
};

use super::Page;

#[derive(Debug)]
pub struct State {
    pub page: Page,
    pub bindings: Vec<Binding>,
    pub current: Option<usize>,
    pub feedback: Option<SessionFeedback>,
    pub progress: Progress,
    pub config_path: PathBuf,
    pub store: Option<ProgressStore>,
    pub error: Option<String>,
    pub challenge_started: Instant,
}

impl State {
    pub fn new(config_path: PathBuf) -> (Self, Task<super::Message>) {
        let store = ProgressStore::discover().ok();
        let progress = store
            .as_ref()
            .and_then(|store| store.load().ok())
            .unwrap_or_default();
        let (bindings, error) = match load_bindings(&config_path) {
            Ok(bindings) if !bindings.is_empty() => (bindings, None),
            Ok(_) => (Vec::new(), Some("No Hyprland bindings were found".into())),
            Err(error) => (Vec::new(), Some(error.to_string())),
        };
        let current = select_next(&bindings, &progress, None);

        (
            Self {
                page: Page::Practice,
                bindings,
                current,
                feedback: None,
                progress,
                config_path,
                store,
                error,
                challenge_started: Instant::now(),
            },
            Task::none(),
        )
    }

    pub fn current_binding(&self) -> Option<&Binding> {
        self.current.and_then(|index| self.bindings.get(index))
    }

    pub fn save(&mut self) {
        if let Some(store) = &self.store {
            if let Err(error) = store.save(&self.progress) {
                self.error = Some(format!("Could not save progress: {error}"));
            }
        }
    }
}
