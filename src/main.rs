mod app;
mod input;
mod learning;
mod omarchy;
mod storage;
mod ui;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    /// Hyprland configuration file or directory to learn from.
    #[arg(short, long)]
    config: Option<std::path::PathBuf>,
}

fn main() -> iced::Result {
    let cli = Cli::parse();
    let config = cli.config.unwrap_or_else(omarchy::default_config_path);

    iced::application("Keyarchy", app::update, ui::view)
        .subscription(app::subscription)
        .theme(app::theme)
        .window_size((1024.0, 680.0))
        .run_with(move || app::State::new(config))
}
