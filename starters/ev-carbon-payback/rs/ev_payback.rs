// EV Carbon Payback
//
// This program reads four whole numbers from standard input, one per line, and
// prints exactly three whole numbers, one per line, and nothing else.
//
// It must not print anything before those three lines, including a prompt. Read
// every value with read_i32(), which prints nothing: a printed prompt would make
// the two languages' output differ and break the comparison the grader does.

use std::io;

// Provided, do not edit. This is the helper the Variables and State lecture had
// you call. It reads one line and hands back the whole number it spells.
fn read_i32() -> i32 {
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("failed to read line");
    text.trim().parse().expect("not a whole number")
}

fn main() {
    // TODO: read the four values below, one per line, with read_i32()
    let extra_manufacturing_gco2 = 0;
    let gasoline_per_km_gco2 = 0;
    let ev_per_km_gco2 = 0;
    let annual_km = 0;

    // TODO: replace each 0 below with the right computation from the variables above.
    let payback_km = 0;
    let whole_years = 0;
    let leftover_km = 0;

    println!("{}", payback_km);
    println!("{}", whole_years);
    println!("{}", leftover_km);
}
