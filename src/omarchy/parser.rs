use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use crate::input::Hotkey;

use super::Binding;

pub fn load_bindings(path: &Path) -> Result<Vec<Binding>> {
    let root = if path.is_dir() {
        path.join("hyprland.conf")
    } else {
        path.to_path_buf()
    };
    let mut variables = default_variables();
    let mut bindings = Vec::new();
    let mut visited = Vec::new();
    parse_file(&root, &mut variables, &mut bindings, &mut visited)?;
    bindings.sort_by(|left, right| left.action.cmp(&right.action));
    bindings.dedup_by(|left, right| left.hotkey == right.hotkey && left.action == right.action);
    Ok(bindings)
}

fn parse_file(
    path: &Path,
    variables: &mut HashMap<String, String>,
    bindings: &mut Vec<Binding>,
    visited: &mut Vec<PathBuf>,
) -> Result<()> {
    let path = path
        .canonicalize()
        .with_context(|| format!("Cannot read {}", path.display()))?;
    if visited.contains(&path) {
        return Ok(());
    }
    visited.push(path.clone());
    let contents =
        fs::read_to_string(&path).with_context(|| format!("Cannot read {}", path.display()))?;

    for raw_line in contents.lines() {
        let line = raw_line.split('#').next().unwrap_or_default().trim();
        if line.is_empty() {
            continue;
        }
        if let Some((name, value)) = parse_variable(line) {
            variables.insert(name, expand(value, variables));
            continue;
        }
        if let Some(source) = line
            .strip_prefix("source")
            .and_then(|rest| rest.trim().strip_prefix('='))
        {
            let expanded = expand(source.trim(), variables);
            let source_path = expand_home(&expanded);
            let source_path = if source_path.is_absolute() {
                source_path
            } else {
                path.parent().unwrap_or(Path::new(".")).join(source_path)
            };
            if source_path.exists() {
                parse_file(&source_path, variables, bindings, visited)?;
            }
            continue;
        }
        if let Some(binding) = parse_binding(line, variables, &path) {
            bindings.push(binding);
        }
    }
    Ok(())
}

fn parse_variable(line: &str) -> Option<(String, &str)> {
    let (name, value) = line.split_once('=')?;
    let name = name.trim();
    if !name.starts_with('$') {
        return None;
    }
    Some((name.to_owned(), value.trim()))
}

fn parse_binding(
    line: &str,
    variables: &HashMap<String, String>,
    source: &Path,
) -> Option<Binding> {
    let (kind, value) = line.split_once('=')?;
    if !kind.trim().starts_with("bind") {
        return None;
    }
    let fields: Vec<_> = value.split(',').map(str::trim).collect();
    if fields.len() < 3 || fields[1].is_empty() {
        return None;
    }
    let modifiers = expand(fields[0], variables);
    let key = expand(fields[1], variables);
    let dispatcher = fields[2];
    let argument = fields.get(3..).unwrap_or_default().join(", ");
    let action = describe_action(dispatcher, &argument);
    Some(Binding {
        hotkey: Hotkey::parse(&modifiers, &key),
        action,
        source: source.display().to_string(),
    })
}

fn describe_action(dispatcher: &str, argument: &str) -> String {
    let dispatcher = dispatcher.trim();
    let argument = argument.trim();
    match dispatcher.to_ascii_lowercase().as_str() {
        "workspace" => format!("Switch to workspace {argument}"),
        "movetoworkspace" => format!("Move window to workspace {argument}"),
        "killactive" => "Close the active window".into(),
        "togglefloating" => "Toggle floating mode".into(),
        "fullscreen" => "Toggle fullscreen".into(),
        "exec" => humanize_command(argument),
        _ if argument.is_empty() => humanize(dispatcher),
        _ => format!("{} {argument}", humanize(dispatcher)),
    }
}

fn humanize_command(command: &str) -> String {
    let executable = command.split_whitespace().next().unwrap_or(command);
    format!(
        "Launch {}",
        executable.rsplit('/').next().unwrap_or(executable)
    )
}

fn humanize(value: &str) -> String {
    let mut output = String::new();
    for (index, character) in value.chars().enumerate() {
        if index > 0 && character.is_uppercase() {
            output.push(' ');
        }
        output.push(character);
    }
    let mut chars = output.chars();
    chars
        .next()
        .map(|first| first.to_uppercase().collect::<String>() + chars.as_str())
        .unwrap_or(output)
}

fn expand(value: &str, variables: &HashMap<String, String>) -> String {
    variables
        .iter()
        .fold(value.trim().to_owned(), |text, (name, replacement)| {
            text.replace(name, replacement)
        })
}

fn expand_home(value: &str) -> PathBuf {
    if let Some(rest) = value.strip_prefix("~/") {
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home).join(rest);
        }
    }
    PathBuf::from(value)
}

fn default_variables() -> HashMap<String, String> {
    HashMap::from([
        ("$mainMod".into(), "SUPER".into()),
        ("$terminal".into(), "terminal".into()),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bindings_and_sources() {
        let directory = tempfile::tempdir().unwrap();
        fs::write(
            directory.path().join("extra.conf"),
            "bind = $mainMod SHIFT, 3, movetoworkspace, 3\n",
        )
        .unwrap();
        fs::write(
            directory.path().join("hyprland.conf"),
            "$mainMod = SUPER\nsource = extra.conf\nbind = $mainMod, Q, killactive,\n",
        )
        .unwrap();
        let bindings = load_bindings(directory.path()).unwrap();
        assert_eq!(bindings.len(), 2);
        assert!(bindings
            .iter()
            .any(|binding| binding.action == "Move window to workspace 3"));
        assert!(bindings
            .iter()
            .any(|binding| binding.hotkey.to_string() == "Super + Q"));
    }
}
