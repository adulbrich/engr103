# Assignment 6 starter. The full specification is the assignment page.
#
# The grader calls your function directly with values it chooses and reads only
# what you return, so nothing is graded on how you get there. Print whatever you
# like while working; printing is never the answer.

def reading_status(reading_ugm3, reference_ugm3, tolerance_ugm3):
    # TODO: return the status code, 0, 1, 2 or 3, following the priority order.
    ...


# ---------------------------------------------------------------------------
# Provided, do not edit. This runs your work so you can try it by hand.
# The grader never runs it; it calls your function directly.
# ---------------------------------------------------------------------------

REFERENCE_UGM3 = 40.0
TOLERANCE_UGM3 = 3.0


def main():
    reading_text = input("Live sensor reading (ug/m3): ")
    try:
        reading = float(reading_text)
        if reading_status(reading, REFERENCE_UGM3, TOLERANCE_UGM3) == 0:
            print("trusted")
        else:
            print("rejected")
    except ValueError:
        print("unreadable")


if __name__ == "__main__":
    main()
