use log::Level;

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}

async fn sleeper() {
    log::info!("Sleeping");
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    log::info!("Awake!");
}
async fn reader() {
    log::info!("Reading some beeg data");
    let contents = tokio::fs::read_to_string("beeg.csv").await.unwrap();

    tokio::task::spawn_blocking(move || {
        fib(40);
    })
    .await
    .unwrap();

    log::info!("Reading beeg {}", contents.len());
}

async fn run() {
    tokio::join!(sleeper(), reader(), reader(), reader(), reader(), reader(),);
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();
    let s = std::time::Instant::now();
    run().await;
    let end = std::time::Instant::now();
    println!("took {:?} sec", end - s);
}
