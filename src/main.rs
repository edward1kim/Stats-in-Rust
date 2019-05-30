mod stats;

use std::io;

fn main() {
    println!("Give data values, separated by spaces. Then press enter.");
    let mut data = String::new();
    io::stdin()
        .read_line(&mut data)
        .expect("Failed to read line");
    let v: Vec<f64> = data
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let summary: Vec<Vec<f64>> = stats::summarize(&v);
    println!("Summary (value : frequency):");
    let mut i = 0;
    while i < summary.len() {
        println!("{} : {}", summary[i][0], summary[i][1]);
        i += 1;
    }
    print!("\n");
    println!("count = {}", stats::count(&v));
    println!("sum = {}", stats::sum(&v));
    println!("mean = {}", stats::mean(&v));
    println!("stdev = {}", stats::stdev(&v));
    println!("median = {}", stats::median(&v));
    println!("mode = {}", stats::mode(&v));
    println!("min = {}", stats::min(&v));
    println!("max = {}", stats::max(&v));
    println!("  0th percentile = {}", stats::percentile(&v, 0.0));
    println!(" 25th percentile = {}", stats::percentile(&v, 0.25));
    println!(" 50th percentile = {}", stats::percentile(&v, 0.5));
    println!(" 75th percentile = {}", stats::percentile(&v, 0.75));
    println!("100th percentile = {}", stats::percentile(&v, 1.0));
}
