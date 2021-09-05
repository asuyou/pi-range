use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::signal;
use tokio::sync::{broadcast, mpsc};
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use tokio_tungstenite::tungstenite::Message;
use tracing_subscriber;
use tracing_subscriber::filter::LevelFilter;
mod args;
mod sensor;
mod shutdown;
mod trigger;

const TRIG: u8 = 23;
const ECHO: u8 = 24;
const SONIC_SPEED: f64 = 0.034; // cm/mico

type FutureResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> FutureResult<()> {
  let args = args::get_args();
  let filter_level: LevelFilter = match args.verbose {
    0 => LevelFilter::OFF,
    1 => LevelFilter::INFO,
    2 => LevelFilter::DEBUG,
    _ => LevelFilter::TRACE,
  };

  tracing_subscriber::fmt()
    .with_max_level(filter_level)
    .init();

  let socket = args.socket.clone();
  let poll = args.poll.clone();
  let distance = args.distance.clone();
  let wait_time = args.wait.clone();

  let (tx, rx) = mpsc::channel::<f64>(10);
  let (notify_shutdown, notify_shutdown_receiver) = broadcast::channel(1);

  // Multi threaded to prevent any blocking
  let _send_thread =
    tokio::spawn(async move { range_trigger(tx, notify_shutdown_receiver, poll, distance, wait_time).await });
  let _web_thread = tokio::spawn(async move { web_socket(rx, socket).await });

  signal::ctrl_c().await?;
  tracing::info!("Cleaning up");
  notify_shutdown.send(()).unwrap();
  tokio::time::sleep(Duration::from_secs(1)).await;

  Ok(())
}

// Handles setting up and running sensor
async fn range_trigger(
  tx: mpsc::Sender<f64>,
  shutdown_rx: broadcast::Receiver<()>,
  poll: u64,
  distance: f64,
  wait_time: u64,
) -> FutureResult<()> {
  let sensor = match sensor::Sensor::new(TRIG, ECHO, SONIC_SPEED, poll) {
    Ok(sensor) => sensor,
    Err(_) => return Ok(()),
  };
  tracing::debug!("Sensor generated");
  let mut main_run = trigger::MainSensor::new(shutdown_rx, tx, sensor, distance, wait_time);
  main_run.run().await;
  tracing::info!("Ranger shutting down");
  Ok(())
}

// Web socket connection handling
async fn web_socket(mut rx: mpsc::Receiver<f64>, url: String) {
  let url = url::Url::parse(&url).unwrap();
  let (ws_stream, _) = tokio_tungstenite::connect_async(url)
    .await
    .expect("Error connecting");

  let (mut write, _read) = ws_stream.split();

  tracing::info!("WebSocket handshake has been successfully completed");
  while let Some(message) = rx.recv().await {
    let send_msg = Message::text(message.to_string());
    tracing::debug!("Message received from channel {}", send_msg);
    write.send(send_msg).await.unwrap();
    tracing::debug!("Message sent via web socket");
  }
  tracing::info!("Shutting down web socket");
  let close_frame = CloseFrame {
    code: CloseCode::Away,
    reason: Default::default(),
  };
  let mut ws_stream = write.reunite(_read).unwrap();
  tracing::debug!("Socket reunited");
  ws_stream.close(Some(close_frame)).await.unwrap();
  tracing::debug!("Socket closed")
}
