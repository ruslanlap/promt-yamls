use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::{input::Hotkey, learning::AttemptResult};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Progress {
    pub xp: u64,
    pub streak: u32,
    pub best_streak: u32,
    pub total_attempts: u32,
    pub correct_attempts: u32,
    pub bindings: HashMap<String, BindingProgress>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BindingProgress {
    pub attempts: u32,
    pub correct: u32,
    pub average_response_ms: u64,
}

impl Progress {
    pub fn record(&mut self, hotkey: &Hotkey, result: &AttemptResult) {
        self.total_attempts += 1;
        self.streak = result.new_streak;
        self.best_streak = self.best_streak.max(self.streak);
        self.xp += u64::from(result.xp);
        if result.correct {
            self.correct_attempts += 1;
        }

        let stat = self.bindings.entry(hotkey.to_string()).or_default();
        let previous_total = stat
            .average_response_ms
            .saturating_mul(u64::from(stat.attempts));
        stat.attempts += 1;
        stat.average_response_ms = (previous_total + result.response_ms) / u64::from(stat.attempts);
        if result.correct {
            stat.correct += 1;
        }
    }

    pub fn accuracy(&self) -> f32 {
        if self.total_attempts == 0 {
            0.0
        } else {
            self.correct_attempts as f32 / self.total_attempts as f32
        }
    }
}

#[derive(Debug)]
pub struct ProgressStore {
    path: PathBuf,
}

impl ProgressStore {
    pub fn discover() -> Result<Self> {
        let directories = ProjectDirs::from("dev", "keyarchy", "Keyarchy")
            .context("Could not determine an XDG data directory")?;
        Ok(Self {
            path: directories.data_local_dir().join("progress.json"),
        })
    }

    pub fn load(&self) -> Result<Progress> {
        if !self.path.exists() {
            return Ok(Progress::default());
        }
        let content = fs::read_to_string(&self.path)
            .with_context(|| format!("Could not read {}", self.path.display()))?;
        serde_json::from_str(&content).context("Progress file is invalid")
    }

    pub fn save(&self, progress: &Progress) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let temporary = self.path.with_extension("json.tmp");
        fs::write(&temporary, serde_json::to_vec_pretty(progress)?)?;
        fs::rename(&temporary, &self.path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::learning::score_attempt;
    use std::time::Duration;

    #[test]
    fn records_attempt_statistics() {
        let hotkey = Hotkey::parse("SUPER", "Q");
        let result = score_attempt(&hotkey, &hotkey, Duration::from_secs(3), 0);
        let mut progress = Progress::default();
        progress.record(&hotkey, &result);
        assert_eq!(progress.correct_attempts, 1);
        assert_eq!(progress.streak, 1);
        assert_eq!(progress.accuracy(), 1.0);
    }
}
