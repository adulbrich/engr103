// Assignment 9 starter. The full specification is the assignment page.
//
// The grader calls your function directly with values it chooses and reads only
// what you return, so nothing is graded on how you get there. Print whatever you
// like while working; printing is never the answer.

fn code_for(reading_ugm3: i32) -> String {
    // TODO: return the three-character base-36 code.
    todo!()
}

fn digit_value(c: char) -> i32 {
    // TODO: return the digit's value, or -1 if c is not a base-36 digit character.
    todo!()
}

// ---------------------------------------------------------------------------
// Provided, do not edit. This runs your work so you can try it by hand.
// The grader never runs main; it calls your function directly.
// ---------------------------------------------------------------------------

use std::io;

// Provided, do not edit. The helper the Variables and State lecture had you call.
fn read_i32() -> i32 {
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("failed to read line");
    text.trim().parse().expect("not a whole number")
}

fn main() {
    println!("Reading (ug/m3): ");
    let reading_ugm3 = read_i32();
    println!("code: {}", code_for(reading_ugm3));
}
