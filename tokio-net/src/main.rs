use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net;

#[tokio::main]
async fn main() {
  let host = "127.0.0.1:8000";
  let stream = net::TcpListener::bind(host).await.unwrap();

  println!("Listening on: {}", host);

  loop {
    // Accepting a new connection
    let (mut sock, _) = stream.accept().await.unwrap();

    // Spawn a new task to handle the connection
    tokio::spawn(async move {
      let mut buf = [0; 1024];

      // In a loop, read data from the socket and write the data back.
      loop {
        let n = match sock.read(&mut buf).await {
          // socket closed
          Ok(n) if n == 0 => return,
          Ok(n) => n,
          Err(e) => {
            eprintln!("failed to read from socket; err = {:?}", e);
            return;
          }
        };

        // Write the data back
        if let Err(e) = sock.write_all(&buf[0..n]).await {
          eprintln!("failed to write to socket; err = {:?}", e);
          return;
        }
      }
    });
  }
}

