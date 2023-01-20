use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Path where be downloaded doujin
    #[arg(short, long)]
    pub path: Option<std::path::PathBuf>,

    /// Force ignoreing cache
    #[arg(short, long)]
    pub force: Option<bool>,

    /// Nhentai code
    #[arg(short, long)]
    pub code: i64,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Show config file
    Config {
        show: Option<bool>,

        /// Set csfrtoken and user-agent from `config.json` file
        edit: Option<bool>,
    },
}
