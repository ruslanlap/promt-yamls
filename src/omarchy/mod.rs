mod bindings;
mod parser;

pub use bindings::Binding;
pub use parser::load_bindings;

use std::path::PathBuf;

pub fn default_config_path() -> PathBuf {
    std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
        .unwrap_or_else(|| PathBuf::from(".config"))
        .join("hypr")
}
