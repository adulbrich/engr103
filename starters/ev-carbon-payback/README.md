# Assignment 3 starter

The full specification is the assignment page on the course site. This README
repeats only the contract, so you have it beside the code.

## The contract

The program reads four whole numbers from standard input, one per
line, in this order:

  1. the EV's extra manufacturing footprint, g CO2e
  2. the gasoline vehicle's operating emissions, g CO2e per km
  3. the EV's operating emissions, g CO2e per km
  4. the typical annual driving distance, km

It then prints exactly three whole numbers, one per line:

  1. the payback distance in km
  2. the complete years of typical driving inside that distance
  3. the leftover km in the partial year

It prints nothing before those three lines, including no prompt.

## What you edit

py/ev_payback.py or rs/ev_payback.rs. Replace each TODO placeholder.

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
