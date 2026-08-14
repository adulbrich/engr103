# Assignment 10 starter

The full specification is the assignment page on the course site. This README
repeats only the contract, so you have it beside the code.

## The contract

Write log_reading(manifest, line). When line is a valid reading it
appends station_id:reading_ugm3 to manifest and returns the report
line LOGGED station_id reading_ugm3. Otherwise it returns exactly
REJECTED and leaves manifest completely unchanged.

The grader reads the report line you return AND inspects the caller's
manifest after the call, so both have to be right.

## What you edit

py/report.py or rs/report.rs. Replace each TODO. Anything marked
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
