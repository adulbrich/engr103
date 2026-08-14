# Assignment 10 starter. The full specification is the assignment page.
#
# The grader calls your function directly with values it chooses and reads only
# what you return, so nothing is graded on how you get there. Print whatever you
# like while working; printing is never the answer.

def log_reading(manifest, line):
    # TODO: return the report line, and append to manifest when the line is valid.
    ...


# ---------------------------------------------------------------------------
# Provided, do not edit. This runs your work so you can try it by hand.
# The grader never runs it; it calls your function directly.
# ---------------------------------------------------------------------------

def main():
    manifest = []
    line = input("Reading line: ")
    print(log_reading(manifest, line))
    print("manifest:", manifest)


if __name__ == "__main__":
    main()
