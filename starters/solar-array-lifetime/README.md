# Assignment 8 starter

The full specification is the assignment page on the course site. This README
repeats only the contract, so you have it beside the code.

## The contract

Write lifetime_generation_kwh(start_output_kwh, degradation_percent,
floor_kwh), returning the total energy the array generates over its
life, in kilowatt-hours, as a float.

A year counts only while its output stays at or above the floor. There
is no exponentiation operator on the ladder, and the number of
qualifying years depends on all three inputs, so the total has to be
built by iterating.

## What you edit

py/array_lifetime.py or rs/array_lifetime.rs. Replace each TODO. Anything marked
"provided, do not edit" is code you were given; the grader supplies its own copy.

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
