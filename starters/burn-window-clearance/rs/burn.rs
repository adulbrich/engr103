// Assignment 5 starter. The full specification is the assignment page.
//
// The grader calls your function directly with values it chooses and reads only
// what you return, so nothing is graded on how you get there. Print whatever you
// like while working; printing is never the answer.

fn burn_clearance_status(
    wind_speed_mph: f64,
    relative_humidity_percent: f64,
    fuel_moisture_percent: f64,
    burn_ban_declared: bool,
) -> i32 {
    // TODO: return the status code, 0, 1, 2 or 3, following the priority order.
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

fn read_i32() -> i32 {
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("failed to read line");
    text.trim().parse().expect("not a whole number")
}

fn main() {
    println!("Wind speed (mph): ");
    let wind_speed_mph = read_f64();
    println!("Relative humidity (%): ");
    let relative_humidity_percent = read_f64();
    println!("Fuel moisture (%): ");
    let fuel_moisture_percent = read_f64();
    println!("Burn ban declared (1 for yes, 0 for no): ");
    let burn_ban_declared = read_i32() == 1;
    println!(
        "status: {}",
        burn_clearance_status(
            wind_speed_mph,
            relative_humidity_percent,
            fuel_moisture_percent,
            burn_ban_declared
        )
    );
}
