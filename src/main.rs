use async_trait::async_trait;
use futures::future::BoxFuture;

#[async_trait]
trait AsyncTrait {
    async fn async_function(&self) -> Result<String, ()>;
}

trait ManualAsyncTrait {
    fn manual_async_function(&self) -> BoxFuture<Result<String, ()>>;
}

struct A;

#[async_trait]
impl AsyncTrait for A {
    async fn async_function(&self) -> Result<String, ()> {
        Ok("async_function".to_string())
    }
}

impl ManualAsyncTrait for A {
    fn manual_async_function(&self) -> BoxFuture<Result<String, ()>> {
        let f = async { Ok("manual_async_function".to_string()) };
        Box::pin(f)
    }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let a = A;
    println!("{}", a.async_function().await?);
    println!("{}", a.manual_async_function().await?);
    Ok(())
}
