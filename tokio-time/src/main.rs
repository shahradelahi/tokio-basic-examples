use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let dur = time::Duration::from_secs(1);

  if let Err(_) = time::timeout(dur, sleepy()).await {
    println!("Tick timed out");
  }

  let mut interval = time::interval(dur);
  for i in 1..6 {
    interval.tick().await;
    println!("Tick {}", i);
  }

  Ok(())
}

async fn sleepy() {
  println!("Sleeping for 2 seconds...");
  time::sleep(time::Duration::from_secs(2)).await;
  println!("Awake!");
}

