# Assignment 9 starter. The full specification is the assignment page.
#
# The grader calls your function directly with values it chooses and reads only
# what you return, so nothing is graded on how you get there. Print whatever you
# like while working; printing is never the answer.

def code_for(reading_ugm3):
    # TODO: return the three-character base-36 code as a string.
    ...


def digit_value(c):
    # TODO: return the digit's value, or -1 if c is not a base-36 digit character.
    ...


# ---------------------------------------------------------------------------
# Provided, do not edit. This runs your work so you can try it by hand.
# The grader never runs it; it calls your function directly.
# ---------------------------------------------------------------------------

def main():
    reading_ugm3 = int(input("Reading (ug/m3): "))
    print("code:", code_for(reading_ugm3))


if __name__ == "__main__":
    main()
