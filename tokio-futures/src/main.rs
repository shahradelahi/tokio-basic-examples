use simple_logger::SimpleLogger;
use tokio::time;

#[tokio::main]
async fn main() {
  SimpleLogger::new().init().unwrap();

  let start = time::Instant::now();
  run().await;
  let end = time::Instant::now();

  log::info!("Elapsed time: {:?}", end - start);
}

async fn run() {
  // run and forget
  tokio::spawn(async {
    sleeper("f1").await;
    log::info!("Hello from a background task");
  });
  sleeper("f2").await;
}

async fn sleeper(name: &str) {
  log::info!("{} is sleeping", name);
  tokio::time::sleep(std::time::Duration::from_secs(1)).await;
  log::info!("{} is awake", name);
}
