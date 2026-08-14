# Assignment 1 starter

The full specification is the assignment page on the course site. This README
repeats only the contract, so you have it beside the code.

## The contract

Both hello.py and hello.rs print exactly one line:

    Hello from the field office.

## What you edit

Nothing, this one is already complete. Run it, then run ./check.

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
