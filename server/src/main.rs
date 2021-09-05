use enigo::*;
use futures::StreamExt;
use itertools::Itertools;
use std::sync::Arc;
use tracing_subscriber;
use tracing_subscriber::filter::LevelFilter;
use warp::ws::WebSocket;
use warp::Filter;
mod args;

const SERVE_IP: [u8; 4] = [192, 168, 0, 5];
const SERVE_PORT: u16 = 3030;

type Sequence = Arc<String>;

#[tokio::main]
async fn main() {
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

  // Need to convert to allow use to pass to socket function
  let seq = Arc::new(args.sequence.clone());
  let seq = warp::any().map(move || seq.clone());
  let do_seq = args.do_action.clone();

  tracing::info!("<-- Web server started -->\n");
  let routes = warp::path("ws")
    // The `ws()` filter will prepare the Websocket handshake.
    .and(warp::ws())
    .and(seq)
    .map(move |ws: warp::ws::Ws, seq: Sequence| {
      // And then our closure will be called when it completes...
      ws.on_upgrade(move |ws| client_connected(ws, seq.clone(), do_seq))
    });

  let ip = format!("{}:{}", SERVE_IP.iter().join("."), SERVE_PORT);
  tracing::info!("Serving on {}", ip);
  tracing::info!("Routes:\n-> /ws/\n\n");

  let routes = warp::get().and(routes);

  warp::serve(routes).run((SERVE_IP, SERVE_PORT)).await;
}

async fn client_connected(ws: WebSocket, seq: Sequence, do_seq: bool) {
  tracing::info!("Client connected");
  let mut enigo = Enigo::new();
  let (_client_write, mut client_read) = ws.split();

  while let Some(result) = client_read.next().await {
    tracing::debug!("Received message");
    let msg = match result {
      Ok(msg) => msg,
      Err(e) => {
        tracing::warn!("{}", e);
        break;
      }
    };
    if let Ok(msg) = msg.to_str() {
      tracing::debug!("{}", msg);
      if do_seq {
        tracing::info!("Doing action");
        enigo.key_sequence_parse(&seq.as_str());
      }
    }
  }
}
