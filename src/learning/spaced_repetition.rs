use crate::{omarchy::Binding, storage::Progress};

/// Selects the least-mastered binding, rotating ties away from the current item.
pub fn select_next(
    bindings: &[Binding],
    progress: &Progress,
    current: Option<usize>,
) -> Option<usize> {
    if bindings.is_empty() {
        return None;
    }
    let offset = current.map_or(0, |index| (index + 1) % bindings.len());
    (0..bindings.len())
        .map(|step| (offset + step) % bindings.len())
        .min_by_key(|index| {
            let stat = progress.bindings.get(&bindings[*index].hotkey.to_string());
            stat.map_or((0_u32, 0_u32), |stat| (stat.correct, stat.attempts))
        })
}
