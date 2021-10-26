use std::{env, fs};

const THRESHOLD: f64 = 1.8;

fn main() {
    let fin = env::args().nth(1).expect("Input file missing");
    let fout = env::args().nth(2).expect("Output file missing");
    let threshold = env::args()
        .nth(3)
        .map(|t| t.trim().parse::<f64>().expect("Failed to parse threshold"))
        .unwrap_or(THRESHOLD);

    let data = fs::read_to_string(fin).expect("Could not open supplied file");

    let new_data = data
        .split("\n")
        .skip(2)
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts = line.split(",").skip(1).collect::<Vec<_>>();
            let ch1 = parts[0].parse::<f64>().expect("column 0: invalid float");
            let ch2 = parts[1].parse::<f64>().expect("column 1: invalid float");
            let ch1b = if is_high(ch1, threshold) { 1 } else { 0 };
            let ch2b = if is_high(ch2, threshold) { 1 } else { 0 };

            println!("{},{} => {},{}", ch1, ch2, ch1b, ch2b);

            format!("{},{}", ch1b, ch2b)
        })
        .collect::<Vec<_>>()
        .join("\n");

    fs::write(fout, new_data).expect("Could not write to outfile");
}

#[inline]
fn is_high(data: f64, threshold: f64) -> bool {
    data > threshold
}
