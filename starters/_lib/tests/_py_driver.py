#!/usr/bin/env python3
"""Load a student's file, call one function, and report the result as JSON.

This runs in its own interpreter, one call per run, so a crash in one case
cannot hide the others and a stray print cannot be mistaken for the answer.
It is part of the check harness. You do not edit or submit this file.
"""

import importlib.util
import io
import json
import sys
import contextlib


def main():
    request = json.load(sys.stdin)

    spec = importlib.util.spec_from_file_location("student", request["source"])
    module = importlib.util.module_from_spec(spec)
    # A student file may guard its own demo behind `if __name__ == "__main__"`,
    # and naming the module "student" keeps that guard shut while we import it.
    with contextlib.redirect_stdout(io.StringIO()):
        spec.loader.exec_module(module)

    name = request["call"]
    if not hasattr(module, name):
        print(f"your file does not define a function named {name}", file=sys.stderr)
        sys.exit(1)

    args = list(request["args"])
    mutates = request.get("mutates")
    with contextlib.redirect_stdout(io.StringIO()):
        value = getattr(module, name)(*args)

    result = {"value": value}
    if mutates is not None:
        # The case declares which argument the function may change in place;
        # report it back so the harness can check what the caller would see.
        result["mutated"] = args[mutates]
    print(json.dumps(result, default=str))


if __name__ == "__main__":
    main()
