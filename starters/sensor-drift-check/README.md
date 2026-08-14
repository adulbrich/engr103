# Assignment 6 starter

The full specification is the assignment page on the course site. This README
repeats only the contract, so you have it beside the code.

## The contract

Write reading_status(reading_ugm3, reference_ugm3, tolerance_ugm3),
returning the status code:

  1  below 0.0, a low-end hardware fault
  2  above 500.0, the sensor has railed high
  3  in range but disagreeing with the reference
  0  in range and agreeing, the only trusted case

The priority order is on the assignment page. The grader calls the
function directly and reads only what you return.

## What you edit

py/classify_reading.py or rs/classify_reading.rs. Replace each TODO. Anything marked
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
