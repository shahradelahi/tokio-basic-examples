use std::error::Error;

use tokio::io::AsyncWriteExt;
use tokio::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut cmd = process::Command::new("sort");

  cmd.stdout(std::process::Stdio::piped());
  cmd.stdin(std::process::Stdio::piped());

  let mut child = cmd.spawn()?;

  let animals: Vec<&str> = vec!["bat", "cat", "rat", "dog", "cow"];

  {
    let mut stdin = child
       .stdin
       .take()
       .expect("child did not have stdin");

    stdin
       .write_all(animals.join(" ").as_bytes())
       .await
       .expect("failed to write to stdin");
  }

  let output = child.wait_with_output().await?;
  println!("sorted: {}", std::str::from_utf8(&output.stdout)?);

  Ok(())
}

