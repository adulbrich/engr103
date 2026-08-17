// Assignment 6 starter. The full specification is the assignment page.
//
// The grader calls your function directly with values it chooses and reads only
// what you return, so nothing is graded on how you get there. Print whatever you
// like while working; printing is never the answer.

fn reading_status(reading_ugm3: f64, reference_ugm3: f64, tolerance_ugm3: f64) -> i32 {
    // TODO: return the status code, 0, 1, 2 or 3, following the priority order.
    todo!()
}

// ---------------------------------------------------------------------------
// Provided, do not edit. This runs your work so you can try it by hand.
// The grader never runs main; it calls your function directly.
// ---------------------------------------------------------------------------

use std::io;

const REFERENCE_UGM3: f64 = 40.0;
const TOLERANCE_UGM3: f64 = 3.0;

fn main() {
    println!("Live sensor reading (ug/m3): ");
    let mut reading_text = String::new();
    io::stdin().read_line(&mut reading_text).expect("failed to read line");

    match reading_text.trim().parse::<f64>() {
        Ok(reading) => {
            if reading_status(reading, REFERENCE_UGM3, TOLERANCE_UGM3) == 0 {
                println!("trusted");
            } else {
                println!("rejected");
            }
        }
        Err(_) => println!("unreadable"),
    }
}
