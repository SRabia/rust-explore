use std::f32::consts::PI;
use num::complex::Complex;

fn dft(input: &[f32]) -> Vec<Complex<f32>> {
    let mut complex_out = Vec::new();
    
    for f_i in 0..input.len() {
        complex_out.push(Complex::new(0.0, 0.0));
        for i_i in 0..input.len() {
            let t = i_i as f32 / input.len() as f32; // Calculate t_i as a floating-point value
            let f = f_i as f32;
            let e = Complex::new(0.0, 2.0 * PI * f * t);
            complex_out[f_i] += Complex::new(input[i_i], 0.0) * e.exp();
        }
    }
    
    complex_out
}

fn sine(window: u32, amplitude: f32, freq: f32) -> Vec<f32>
{
    let mut wave = Vec::new();
    for i in 0..window{
        let window_f:f32 = window as f32;
        let t:f32 = (i as f32) / window_f;
        let value = amplitude*(2.0*PI*freq*t).sin();
        wave.push(value);
    }
    return wave;
}

fn _display_ascii_sine(window: u32, amplitude: f32, freq: f32)-> Vec<f32>{
    let mut wave = Vec::new();
    for i in 0..window{
        let window_f:f32 = window as f32;
        let t:f32 = (i as f32) / window_f;
        let value = amplitude*(2.0*PI*freq*t).sin();
        wave.push(value);

        let bound = (value + amplitude + 0.1)/2.0 * window_f;
        let bound = bound as u32;
        for _ in 0..bound{
            print!("*");
        }
        print!("\n");
    }
    return wave;

}
// fn plot_discrete_ascii(fx: &[f32]) {
//     let max = fx.iter().copied().fold(f32::NEG_INFINITY, f32::max); // Find the maximum value

//     // Create a new iterator with normalized values
//     let normalized_values = fx.iter().map(|&value| value / max);

//     for value_norm in normalized_values {
//         // Do something with the normalized value
//         println!("Normalized value: {}", value_norm);
//     }
// }
    // let config_max = Some(3u8);
    // if let Some(max) = config_max {
    //     println!("The maximum is configured to be {}", max);
    // }

fn plot_discrete_ascii(fx: &[f32], height: u32){
    let max = fx.iter().copied().fold(f32::NEG_INFINITY, f32::max); // Find the maximum value
    let min = fx.iter().copied().fold(f32::INFINITY, f32::max); // Find the maximum value
    let norm = fx.iter().map(|&value| value / max);
    println!("{:?}", norm);
    // dbg!("{}", normalized_values);
    
    //display
    for value in norm{
        let plot_point = value * (height as f32) + min.abs();
        let plot_point = plot_point as u32;
        for _ in 0..plot_point{
            print!(".");
        }
        print!("\n");
    }

}

fn main() {
    println!("hello");
    let sinwave = sine(30, 1.0, 1.0);
    plot_discrete_ascii(&sinwave, 30);
    // let mut wave: Vec<f32>= display_ascii_sine(30, 1.0, 1.0);
    // let mut wav2 = display_ascii_sine(30, 1.0, 1.0);
    // wave.append(&mut wav2);
    // let outdft: Vec<Complex<f32>> = dft(&wave);

    // for data in  outdft {
    //     println!("{}", data.norm());
    // }




    // display_ascii_sine(30, 2.0, 1.0);

    // for i in 0..N{
    //     let t:f32 = (i as f32) / N as f32;
    //     let value = (2.0*PI*P*t).sin();
    //     // println!("value {}{}", t, value);
    //     wave.push(value);
    // }

    // for element in wave{
    //     // println!("{}", element + 1.0)
    //     let bound = (element + 1.1)/2.0 * (N as f32);
    //     let bound = bound as u32;
    //     // print!("{} + 1.0 / 2.0 * {} = {}  ", element, N, bound);
    //     // print!("{}  {}, ", element + 1.0, bound);
    //     for _ in 0..bound{
    //         print!("*");
    //     }
    //     print!("\n");
    //     // print!("({})\n", (element + 1.0)/2.0 * 30.0);
    // }
}
