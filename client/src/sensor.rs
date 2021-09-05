use rppal::gpio::Gpio;
use std::time::{Duration, Instant};

pub struct Sensor {
  sonic_speed: f64,
  trig_pin: rppal::gpio::OutputPin,
  echo_pin: rppal::gpio::InputPin,
  poll_time: u64,
}

impl Sensor {
  pub fn new(
    trig: u8,
    echo: u8,
    sonic_speed: f64,
    poll: u64,
  ) -> Result<Sensor, Box<dyn std::error::Error>> {
    let sensor = Sensor {
      sonic_speed,
      trig_pin: Gpio::new()?.get(trig)?.into_output(),
      echo_pin: Gpio::new()?.get(echo)?.into_input(),
      poll_time: poll,
    };
    Ok(sensor)
  }

  // Polls distance from sensor
  pub async fn poll_distance(&mut self) -> f64 {
    self.trig_pin.set_low();
    tracing::debug!("Waiting for pin to settle");
    tokio::time::sleep(Duration::from_millis(self.poll_time)).await;
    tracing::debug!("Pin settled");

    tracing::debug!("Triggering pin");
    self.trig_pin.set_high();
    tokio::time::sleep(Duration::from_micros(10)).await;
    self.trig_pin.set_low();

    tracing::debug!("Awaiting rise off echo pin");
    self.echo_pin.poll_interrupt(false, None).unwrap();
    let pulse_start = Instant::now();

    tracing::debug!("Awaiting fall of echo pin");
    self.echo_pin.poll_interrupt(false, None).unwrap();
    let pulse_duration = pulse_start.elapsed().as_micros();

    let distance = (self.sonic_speed * pulse_duration as f64) / 2.0;
    tracing::debug!("Distance {}", distance);
    return distance;
  }

  // Updates pin provided to allow falling/rising edge interrupts
  pub async fn update_echo(&mut self) {
    self
      .echo_pin
      .set_interrupt(rppal::gpio::Trigger::Both)
      .unwrap();
    tracing::debug!("Echo pin interrupt has been set");
  }
}
