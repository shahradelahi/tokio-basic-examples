use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  Ok(())
}

async fn sleepy() {
  println!("Sleeping for 2 seconds...");
  time::sleep(time::Duration::from_secs(2)).await;
  println!("Awake!");
}

async fn slowed_number()-> i32 {
  time::sleep(time::Duration::from_secs(2)).await;
  42
}

#[tokio::test]
async fn it_works() {
  sleepy().await;
  assert_eq!(2 + 2, 4);
}

#[tokio::test]
async fn also_it_works() {
  let x = slowed_number().await;
  assert_eq!(x, 42);
}
