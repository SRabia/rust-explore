use futures::executor::block_on;

enum Poll<T> {
    Ready(T),
    Pending,
}

trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

async fn hello_world() {
    println!("hello world");
}

fn main() {
    let future = hello_world();
    block_on(future);
}
