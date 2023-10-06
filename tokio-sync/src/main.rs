use tokio::sync;

#[tokio::main]
async fn main() {

  // ========================== Mutex - Read/Write - Lock - Semaphore ==========================

  let lock = std::sync::Arc::new(
    sync::Mutex::new(MyStruct { field: 0 })
  );

  let lock_a = lock.clone();
  let lock_b = lock.clone();

  let a = tokio::spawn(async move {
    let mut lock = lock_a.lock().await;
    lock.field = 1;
  });

  let b = tokio::spawn(async move {
    let mut lock = lock_b.lock().await;
    lock.field = 2;
  });

  tokio::join!(a,b).0.unwrap();

  let val = lock.lock().await;
  println!("val: {}", val.field);

  // ========================== Channel ==========================

  let (tx, mut rx) = sync::mpsc::channel(32);

  tokio::spawn(async move {
    tx.send(MyStruct { field: 42 }).await.unwrap();
  });

  let my_struct = rx.recv().await.unwrap();
  println!("my_struct.field = {}", my_struct.field);
}

struct MyStruct {
  field: u32,
}

