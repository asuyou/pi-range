use clap::{AppSettings, Clap};

/// A client that connects to a web socket and interacts with a HC-SR04 via GPIO on a raspberry pi
#[derive(Clap)]
#[clap(version = "0.1", author = "asyo <a@asyo.dev>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
  /// Sets websocket to connects to
  #[clap(short, long, default_value = "ws://192.168.0.2:3030/ws")]
  pub socket: String,
  /// Sets interval to poll HC-SR04 (ms). I advise not going below 250
  #[clap(short, long, default_value = "250")]
  pub poll: u64,
  /// Sets distance that that when below will write to socket (cm) (set to `0.0` to always send)
  #[clap(short, long, default_value = "50")]
  pub distance: f64,
  /// Time to wait after a signal had been sent to the websocket
  #[clap(short, long, default_value = "0")]
  pub wait: u64,
  /// A level of verbosity, and can be used multiple times
  #[clap(short, long, parse(from_occurrences))]
  pub verbose: i32,
}

pub fn get_args() -> Opts {
  let opts: Opts = Opts::parse();
  return opts;
}
