// Low-Carbon Concrete
//
// Write the two functions below. The grader calls each one directly with values
// it chooses and reads only what you return, so nothing is graded on how you get
// there. Print whatever you like while working; printing is never the answer.
//
// The two equations are given on the assignment page. You do not derive them.
// How you turn the four arguments into M and C, and what intermediate values you
// give names to along the way, is your decision.

fn fly_ash_mass_kg(
    binder_target_tonnes: f64,
    co2_target_kgco2_per_tonne: f64,
    flyash_kgco2_per_kg: f64,
    slag_kgco2_per_kg: f64,
) -> f64 {
    // TODO: return the fly ash mass in kilograms.
    todo!()
}

fn slag_mass_kg(
    binder_target_tonnes: f64,
    co2_target_kgco2_per_tonne: f64,
    flyash_kgco2_per_kg: f64,
    slag_kgco2_per_kg: f64,
) -> f64 {
    // TODO: return the slag mass in kilograms.
    todo!()
}

// ---------------------------------------------------------------------------
// Provided, do not edit. This runs your functions so you can try them by hand,
// and read_f64 is the helper the Variables and State lecture had you call.
// The grader never runs main; it calls your functions directly.
// ---------------------------------------------------------------------------

use std::io;

fn read_f64() -> f64 {
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("failed to read line");
    text.trim().parse().expect("not a number")
}

fn main() {
    println!("Binder mass target (tonnes): ");
    let binder_target_tonnes = read_f64();
    println!("CO2 target (kg CO2 per tonne): ");
    let co2_target_kgco2_per_tonne = read_f64();
    println!("Fly ash intensity (kg CO2 per kg): ");
    let flyash_kgco2_per_kg = read_f64();
    println!("Slag intensity (kg CO2 per kg): ");
    let slag_kgco2_per_kg = read_f64();

    println!(
        "fly ash kg: {}",
        fly_ash_mass_kg(
            binder_target_tonnes,
            co2_target_kgco2_per_tonne,
            flyash_kgco2_per_kg,
            slag_kgco2_per_kg
        )
    );
    println!(
        "slag kg: {}",
        slag_mass_kg(
            binder_target_tonnes,
            co2_target_kgco2_per_tonne,
            flyash_kgco2_per_kg,
            slag_kgco2_per_kg
        )
    );
}
