use crate::sensor::Sensor;
use crate::shutdown::Shutdown;
use tokio::sync::{broadcast, mpsc};
use std::time::Duration;

pub struct MainSensor {
  sensor: Sensor,
  shutdown: Shutdown,
  result: mpsc::Sender<f64>,
  max_distance: f64,
  wait_time: u64,
}

impl MainSensor {
  pub fn new(
    shutdown_rx: broadcast::Receiver<()>,
    result: mpsc::Sender<f64>,
    sensor: Sensor,
    max_distance: f64,
    wait_time: u64,
  ) -> MainSensor {
    MainSensor {
      shutdown: Shutdown::new(shutdown_rx),
      sensor,
      result,
      max_distance,
      wait_time
    }
  }

  // Handles running of sensor
  pub async fn run(&mut self) {
    self.sensor.update_echo().await;
    while !self.shutdown.is_shutdown() {
      // Selects which ever future finishes first
      let distance = tokio::select! {
          distance = self.sensor.poll_distance() => distance,
          _ = self.shutdown.recv() => {return;}
      };

      if distance <= self.max_distance || self.max_distance == 0.0 {
        tracing::debug!("Forwarding message to websocket");
        self.result.send(distance).await.unwrap();

        tracing::trace!("Started wait after msg to websocket");
        tokio::time::sleep(Duration::from_secs(self.wait_time)).await;
        tracing::trace!("Wait ended after msg to websocket");
      }
    }
    tracing::debug!("Shutdown signal received by run function")
  }
}
