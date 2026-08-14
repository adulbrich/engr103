# Assignment 8 starter. The full specification is the assignment page.
#
# The grader calls your function directly with values it chooses and reads only
# what you return, so nothing is graded on how you get there. Print whatever you
# like while working; printing is never the answer.

def lifetime_generation_kwh(start_output_kwh, degradation_percent, floor_kwh):
    # TODO: return the array's total lifetime output in kWh, as a float.
    ...


# ---------------------------------------------------------------------------
# Provided, do not edit. This runs your work so you can try it by hand.
# The grader never runs it; it calls your function directly.
# ---------------------------------------------------------------------------

def main():
    start_output_kwh = float(input("First-year output (kWh): "))
    degradation_percent = float(input("Degradation per year (%): "))
    floor_kwh = float(input("Warranty floor (kWh): "))
    print("lifetime kWh:", lifetime_generation_kwh(
        start_output_kwh, degradation_percent, floor_kwh))


if __name__ == "__main__":
    main()
