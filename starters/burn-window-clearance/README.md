# Assignment 5 starter

The full specification is the assignment page on the course site. This README
repeats only the contract, so you have it beside the code.

## The contract

Write burn_clearance_status(wind_speed_mph, relative_humidity_percent,
fuel_moisture_percent, burn_ban_declared), returning the status code:

  3  a county burn ban is in effect, outranking every reading
  2  conditions are unsafe
  1  conditions are marginal
  0  the unit is cleared to burn

The priority order is on the assignment page. The grader calls the
function directly and reads only what you return.

## What you edit

py/burn.py or rs/burn.rs. Replace each TODO. Anything marked
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
