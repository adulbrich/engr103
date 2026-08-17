// Assignment 8 starter. The full specification is the assignment page.
//
// The grader calls your function directly with values it chooses and reads only
// what you return, so nothing is graded on how you get there. Print whatever you
// like while working; printing is never the answer.

fn lifetime_generation_kwh(
    start_output_kwh: f64,
    degradation_percent: f64,
    floor_kwh: f64,
) -> f64 {
    // TODO: return the array's total lifetime output in kWh.
    todo!()
}

// ---------------------------------------------------------------------------
// Provided, do not edit. This runs your work so you can try it by hand.
// The grader never runs main; it calls your function directly.
// ---------------------------------------------------------------------------

use std::io;

// Provided, do not edit. The helper the Variables and State lecture had you call.
fn read_f64() -> f64 {
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("failed to read line");
    text.trim().parse().expect("not a number")
}

fn main() {
    println!("First-year output (kWh): ");
    let start_output_kwh = read_f64();
    println!("Degradation per year (%): ");
    let degradation_percent = read_f64();
    println!("Warranty floor (kWh): ");
    let floor_kwh = read_f64();
    println!(
        "lifetime kWh: {}",
        lifetime_generation_kwh(start_output_kwh, degradation_percent, floor_kwh)
    );
}
