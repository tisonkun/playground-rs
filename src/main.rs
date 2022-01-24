use std::future::Future;

#[derive(Debug, Default)]
pub struct Container {
    v: Vec<u8>,
}

impl Container {
    async fn mutation(&mut self) {
        self.v.push(42);
    }

    async fn mutations(&mut self) {
        let n = self.v.len() + 1;
        for _ in 0..n {
            self.mutation().await;
        }
    }

    async fn generic_mutations<F, Fut>(&mut self, f: F)
    where
        F: Fn(&mut Self) -> Fut,
        Fut: Future<Output = ()>,
    {
        let n = self.v.len() + 1;
        for _ in 0..n {
            f(self).await;
        }
    }
}

#[tokio::main]
async fn main() {
    {
        let mut c = Container::default();
        c.mutations().await;
        c.mutations().await;
        println!("{:?}", c);
    }

    {
        let mut c = Container::default();
        c.generic_mutations(|con| con.mutation()).await;
        c.generic_mutations(|con| con.mutation()).await;
        println!("{:?}", c);
    }
}
