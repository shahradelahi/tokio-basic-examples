use tokio::task;

#[tokio::main]
async fn main() {
  let a = task::spawn_blocking(|| {
    println!("Starting fib(30) computation...");
    let result = fib(30);
    println!("fib(30) = {}", result);
  });

  let b = task::spawn_blocking(|| {
    println!("Starting fib(40) computation...");
    let result = fib(40);
    println!("fib(40) = {}", result);
  });

  tokio::join!(a, b).0.unwrap();
}

fn fib(n: u64) -> u64 {
  match n {
    0 => 0,
    1 => 1,
    n => fib(n - 1) + fib(n - 2),
  }
}

