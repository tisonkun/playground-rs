#[async_trait::async_trait]
trait AsyncTrait {
    async fn async_function(&mut self) -> Result<String, ()>;
}

trait ManualAsyncTrait {
    fn manual_async_function(&mut self) -> futures::future::BoxFuture<Result<String, ()>>;
}

struct A(u8);

#[async_trait::async_trait]
impl AsyncTrait for A {
    async fn async_function(&mut self) -> Result<String, ()> {
        println!("async_function: {}", self.0);
        if self.0.saturating_sub(1) > 0 {
            self.0 = self.0.saturating_sub(1);
            self.manual_async_function().await
        } else {
            Ok("async_function".to_string())
        }
    }
}

impl ManualAsyncTrait for A {
    fn manual_async_function(&mut self) -> futures::future::BoxFuture<Result<String, ()>> {
        let f = async {
            println!("manual_async_function: {}", self.0);
            if self.0.saturating_sub(1) > 0 {
                self.0 = self.0.saturating_sub(1);
                self.async_function().await
            } else {
                Ok("manual_async_function".to_string())
            }
        };
        Box::pin(f)
    }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("{}", A(10).async_function().await?);
    println!("{}", A(10).manual_async_function().await?);
    Ok(())
}
