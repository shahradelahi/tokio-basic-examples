use std::error::Error;

use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut file = fs::File::create("foo.txt").await?;
  let mut contents = String::new();

  file.write_all(b"Hello, world!").await?;

  let mut file = fs::File::open("foo.txt").await?;

  file.read_to_string(&mut contents).await?;

  println!("contents: {}", contents);

  Ok(())
}

