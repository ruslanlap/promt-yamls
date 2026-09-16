# Keyarchy

Keyarchy is a Wayland-first desktop trainer for learning Omarchy and Hyprland
keyboard shortcuts. It reads the shortcuts you actually use, presents focused
practice challenges, and stores progress locally.

## Features

- Native Rust desktop UI powered by [Iced](https://iced.rs/).
- Parses `bind*` declarations, variables, and recursively sourced Hyprland
  configuration files.
- Captures shortcuts only while the trainer has focus—no global keylogger and
  no compositor-specific permissions.
- Adaptive practice prioritizes shortcuts with the lowest mastery.
- XP, response-time scoring, streaks, per-shortcut accuracy, and weak-key view.
- XDG-compliant JSON progress storage with atomic writes.
- Dark Tokyo Night theme suitable for the Omarchy desktop.

## Install and run

Install the Arch build dependencies and run from source:

```bash
sudo pacman -S --needed base-devel rust
cargo run --release
```

By default Keyarchy reads `~/.config/hypr/hyprland.conf`. Pass a different
file or Hyprland configuration directory when needed:

```bash
cargo run --release -- --config ~/.config/hypr/hyprland.conf
```

While practicing, keep the Keyarchy window focused. The compositor may still
consume reserved shortcuts before the application receives them. For a fully
isolated session, use a temporary Hyprland submap or duplicate a binding with
an unreserved training combination.

## Data and privacy

Keyarchy does not use network access and does not capture keyboard input while
unfocused. Progress is saved under the platform XDG data directory, normally:

```text
~/.local/share/keyarchy/Keyarchy/progress.json
```

## Development

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

The application is split into state/update logic, UI pages, input
normalization, Hyprland parsing, learning/scoring, and persistence modules under
`src/`.

## Arch package

`packaging/arch/PKGBUILD` is a source-package template. Update its source URL
and checksum after publishing a tagged release, then build with `makepkg -si`.

## License

MIT
