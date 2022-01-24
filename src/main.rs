use std::sync::Arc;

use tokio::sync::Mutex;

pub type Container = Arc<Mutex<u8>>;

async fn outer(c: &Container) -> u8 {
    let mut guard = c.lock().await;
    let result = inner(c).await;
    *guard += 1;
    drop(guard);
    result
}

async fn inner(c: &Container) -> u8 {
    let mut guard = c.lock().await;
    *guard += 1;
    *guard
}

#[tokio::main]
async fn main() {
    let c = Arc::new(Mutex::new(0_u8));
    println!("{:?}", outer(&c).await);
}
