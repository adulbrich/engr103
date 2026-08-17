# Assignment 7 starter

The full specification is the assignment page on the course site. This README
repeats only the contract, so you have it beside the code.

## The contract

Write test_dashboard_cleared(), a test function whose body is a sequence of
assertions checking dashboard_cleared against its specification, ending with a
printed confirmation line once every assertion has held.

You do not write dashboard_cleared, and you do not fix it. It is supplied when
your suite runs, so do not define it in your file.

Your suite is run against the correct implementation, where every assertion must
hold, and against several broken ones, where each must fail at least one of your
assertions. A suite that passes the correct implementation but lets a broken one
through has not certified the check.

## What you edit

py/certify.py or rs/certify.rs. Nothing else.

## Checking your work

    ./check          run your suite against the visible implementations
    ./check py       only the Python side
    ./check rs       only the Rust side

On Windows, where ./check does not run a script directly, use `python3 check`.

You can see three broken versions here. The grader holds more that you cannot
see, so a green check is necessary but not a guarantee: a suite that pins every
boundary in the specification catches versions it was never shown.

## Submitting

    ./pack

That writes submission.zip. Upload it to the Gradescope assignment linked on
Canvas.
