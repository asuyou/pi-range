use clap::{AppSettings, Clap};

/// A server that can perform keyboard actions when a message is received or can be extended to use
/// that data in any way
#[derive(Clap)]
#[clap(version = "0.1", author = "asyo <a@asyo.dev>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
  /// Sets keyboard sequence when a value is received
  #[clap(short, long, default_value = "/")]
  pub sequence: String,
  /// Sets whether to do the keyboard sequence
  #[clap(short, long)]
  pub do_action: bool,
  /// A level of verbosity, and can be used multiple times
  #[clap(short, long, parse(from_occurrences))]
  pub verbose: i32,
}

pub fn get_args() -> Opts {
  let opts: Opts = Opts::parse();
  return opts;
}
