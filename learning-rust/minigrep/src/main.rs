use std::env;
use minigrep::Config;
use std::process;


fn main() {
    // let args:Vec<String> = env::args().collect();
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("in file {}", config.file_path);

    //we don't want to unwrap since the trun function
    //donesnt' return a value that we want to use in the same 
    //way Config::build does. We only care about error detection.
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }





    // let buffer: &mut [i32];
    // let coefficients: [i64; 12];
    // let qlp_shift: i16;

    // for i in 12..buffer.len(){
    //     let prediction = coefficients.iter()
    //         .zip(&buffer[i - 12..i])
    //         .map(|(&c, &s)| c * s as i64)
    //         .sum::<i64>() >> qlp_shift;
    //     let delta = buffer[i];
    //     buffer[i] = prediction as i32 + delta;
    // }

}



