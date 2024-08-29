use std::thread::sleep;
use std::time::{Duration, Instant};

use rodio::source::SineWave;
use rodio::{OutputStream, Sink, Source};
use std::env;

const DEFAULT_TIMEOUT_SECS: u64 = 10 * 60;
const DEFAULT_BEEP_DURATION: f32 = 1.25;

fn print_duration(time: Duration) {
    let remaining_min: u64 = time.as_secs() / 60;
    let remaining_s: u64 = time.as_secs() % 60;

    println!("time:{}:{} mins", remaining_min, remaining_s);
    print!("{esc}c", esc = 27 as char);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut timeout_secs: u64 = DEFAULT_TIMEOUT_SECS;
    if args.len() > 1 {
        timeout_secs = args[1].parse::<u64>().expect("wrong args") * 60;
    }

    let duration: Duration = Duration::new(timeout_secs, 0);
    let now: Instant = Instant::now();

    while duration > now.elapsed() {
        let remaining: Duration = duration - now.elapsed();
        print_duration(remaining);
        sleep(Duration::new(1, 0));
    }
    print!("{esc}c", esc = 27 as char);
    println!("beep!");

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink: Sink = Sink::try_new(&stream_handle).unwrap();

    let source = SineWave::new(440.0)
        .take_duration(Duration::from_secs_f32(DEFAULT_BEEP_DURATION))
        .amplify(0.20);

    sink.append(source);

    sink.sleep_until_end();
}
