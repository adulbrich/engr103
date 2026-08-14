# Assignment 5 starter. The full specification is the assignment page.
#
# The grader calls your function directly with values it chooses and reads only
# what you return, so nothing is graded on how you get there. Print whatever you
# like while working; printing is never the answer.

def burn_clearance_status(wind_speed_mph, relative_humidity_percent,
                          fuel_moisture_percent, burn_ban_declared):
    # TODO: return the status code, 0, 1, 2 or 3, following the priority order.
    ...


# ---------------------------------------------------------------------------
# Provided, do not edit. This runs your work so you can try it by hand.
# The grader never runs it; it calls your function directly.
# ---------------------------------------------------------------------------

def main():
    wind_speed_mph = float(input("Wind speed (mph): "))
    relative_humidity_percent = float(input("Relative humidity (%): "))
    fuel_moisture_percent = float(input("Fuel moisture (%): "))
    burn_ban_declared = input("Burn ban declared (yes/no): ").strip() == "yes"
    print("status:", burn_clearance_status(
        wind_speed_mph, relative_humidity_percent,
        fuel_moisture_percent, burn_ban_declared))


if __name__ == "__main__":
    main()
