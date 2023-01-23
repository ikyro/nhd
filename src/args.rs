use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
  /// Path where be downloaded doujin by default is `~/.nhentai/`
  #[arg(short, long, default_value_t = false)]
  pub path: bool,

  /// Force download doujin
  #[arg(short, long, default_value_t = false)]
  pub force: bool,

  /// Nhentai code
  #[arg(short, long)]
  pub code: i64,
}
