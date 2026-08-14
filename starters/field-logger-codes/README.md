# Assignment 9 starter

The full specification is the assignment page on the course site. This README
repeats only the contract, so you have it beside the code.

## The contract

Write two functions:

  code_for(reading_ugm3)  -> the reading's three-character base-36
                             code, most significant character first
  digit_value(c)          -> the value one base-36 digit character
                             stands for, or -1 if it is not one

They are two halves of one codec, not a matched pair to test against
each other. The grader calls each directly and reads what you return.

## What you edit

py/field_codes.py or rs/field_codes.rs. Replace each TODO. Anything marked
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
