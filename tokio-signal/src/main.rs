use std::error::Error;

use tokio::signal;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

  println!("Waiting for CTRL+C or SIGTERM");

  // Create a stream of SIGINT notifications.
  let mut stream = signal::unix::signal(signal::unix::SignalKind::interrupt())?;

  // Block the current thread until `CTRL+C` or `SIGTERM` is received.
  stream.recv().await;

  println!("Got signal, exiting...");

  Ok(())
}

