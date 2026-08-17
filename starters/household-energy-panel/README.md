# Assignment 2 starter

The full specification is the assignment page on the course site. This README
repeats only the contract, so you have it beside the code.

## The contract

The program prints exactly five lines and nothing else:

  1. the text ENERGY AUDIT (already written)
  2. annual energy use in kWh, as a decimal number
  3. annual emissions in kg CO2e, as a decimal number
  4. the typical-use flag, 1 or 0
  5. the meter status code, a whole number

Every line prints the same characters in both languages, which is
why line 4 is a flag rather than a boolean and why both decimal
readings land on a real fraction.

## What you edit

py/energy_panel.py or rs/energy_panel.rs. Replace each TODO placeholder.

Anything marked "provided, do not edit" is code you were given. The grader
supplies its own copy, so editing it changes nothing for your grade.

## Checking your work

    ./check          run the visible tests on whatever you have written
    ./check py       only the Python side
    ./check rs       only the Rust side

On Windows, where ./check does not run a script directly, use `python3 check`.

These are the visible cases. The grader runs them plus hidden cases you cannot
see, so a green check here is necessary but not a guarantee.

## Submitting

    ./pack

That writes submission.zip. Upload it to the Gradescope assignment linked on
Canvas. You may submit as often as you like; the last one before the deadline
counts.
