// Assignment 10 starter. The full specification is the assignment page.
//
// The grader calls your function directly with values it chooses and reads only
// what you return, so nothing is graded on how you get there. Print whatever you
// like while working; printing is never the answer.

fn log_reading(manifest: &mut Vec<String>, line: &str) -> String {
    // TODO: return the report line, and push onto manifest when the line is valid.
    todo!()
}

// ---------------------------------------------------------------------------
// Provided, do not edit. This runs your work so you can try it by hand.
// The grader never runs main; it calls your function directly.
// ---------------------------------------------------------------------------

use std::io;

fn main() {
    let mut manifest: Vec<String> = Vec::new();
    println!("Reading line: ");
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("failed to read line");
    println!("{}", log_reading(&mut manifest, line.trim()));
    println!("manifest: {:?}", manifest);
}
