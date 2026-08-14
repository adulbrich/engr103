# Assignment 4 starter

The full specification is the assignment page on the course site. This README
repeats only the contract, so you have it beside the code.

## The contract

Write two functions, both returning a float:

  fly_ash_mass_kg(binder_target_tonnes, co2_target_kgco2_per_tonne,
                  flyash_kgco2_per_kg, slag_kgco2_per_kg)
      -> the fly ash mass in kilograms

  slag_mass_kg(binder_target_tonnes, co2_target_kgco2_per_tonne,
               flyash_kgco2_per_kg, slag_kgco2_per_kg)
      -> the slag mass in kilograms

The two equations are given on the assignment page in full. You do not derive or
solve anything. The binder mass target arrives in tonnes, not kilograms, and the
CO2 target arrives as a rate rather than a total; turning the four arguments into
M and C is your work, as is deciding what intermediate values to name.

The grader calls each function directly and reads only what you return. Because
the results are floating-point, it accepts any answer within a small tolerance,
so you never compare floats yourself.

## What you edit

py/mix.py or rs/mix.rs. Replace each TODO. Anything marked "provided, do not
edit" is code you were given; the grader supplies its own copy.

## Checking your work

    ./check          run the visible tests on whatever you have written
    ./check py       only the Python side
    ./check rs       only the Rust side

On Windows, where ./check does not run a script directly, use `python3 check`.

These are the visible cases, the worked example from the assignment page. The
grader runs them plus hidden cases you cannot see, so a green check here is
necessary but not a guarantee.

## Submitting

    ./pack

That writes submission.zip. Upload it to the Gradescope assignment linked on
Canvas. You may submit as often as you like; the last one before the deadline
counts.
