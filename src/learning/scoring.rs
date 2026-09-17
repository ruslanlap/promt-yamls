use std::time::Duration;

use crate::input::Hotkey;

#[derive(Debug, Clone)]
pub struct AttemptResult {
    pub correct: bool,
    pub xp: u32,
    pub response_ms: u64,
    pub new_streak: u32,
}

#[derive(Debug, Clone)]
pub struct SessionFeedback {
    pub correct: bool,
    pub title: String,
    pub detail: String,
}

impl From<AttemptResult> for SessionFeedback {
    fn from(result: AttemptResult) -> Self {
        if result.correct {
            Self {
                correct: true,
                title: "Correct!".into(),
                detail: format!(
                    "+{} XP  •  Streak {}  •  {:.1}s",
                    result.xp,
                    result.new_streak,
                    result.response_ms as f32 / 1000.0
                ),
            }
        } else {
            Self {
                correct: false,
                title: "Not quite".into(),
                detail: "Review the shortcut, then try the next challenge.".into(),
            }
        }
    }
}

pub fn score_attempt(
    expected: &Hotkey,
    pressed: &Hotkey,
    elapsed: Duration,
    streak: u32,
) -> AttemptResult {
    let correct = expected == pressed;
    let response_ms = elapsed.as_millis().min(u64::MAX as u128) as u64;
    let speed_bonus = if response_ms < 2_000 {
        5
    } else if response_ms < 5_000 {
        2
    } else {
        0
    };
    AttemptResult {
        correct,
        xp: if correct { 10 + speed_bonus } else { 0 },
        response_ms,
        new_streak: if correct { streak + 1 } else { 0 },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fast_correct_attempt_gets_bonus() {
        let hotkey = Hotkey::parse("SUPER SHIFT", "3");
        let result = score_attempt(&hotkey, &hotkey, Duration::from_millis(900), 6);
        assert!(result.correct);
        assert_eq!(result.xp, 15);
        assert_eq!(result.new_streak, 7);
    }

    #[test]
    fn mistake_breaks_streak() {
        let expected = Hotkey::parse("SUPER", "Q");
        let pressed = Hotkey::parse("SUPER", "W");
        let result = score_attempt(&expected, &pressed, Duration::ZERO, 4);
        assert!(!result.correct);
        assert_eq!(result.xp, 0);
        assert_eq!(result.new_streak, 0);
    }
}
