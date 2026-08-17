# Low-Carbon Concrete
#
# Write the two functions below. The grader calls each one directly with values
# it chooses and reads only what you return, so nothing is graded on how you get
# there. Print whatever you like while working; printing is never the answer.
#
# The two equations are given on the assignment page. You do not derive them.
# How you turn the four arguments into M and C, and what intermediate values you
# give names to along the way, is your decision.


def fly_ash_mass_kg(binder_target_tonnes, co2_target_kgco2_per_tonne,
                    flyash_kgco2_per_kg, slag_kgco2_per_kg):
    # TODO: return the fly ash mass in kilograms, as a float.
    ...


def slag_mass_kg(binder_target_tonnes, co2_target_kgco2_per_tonne,
                 flyash_kgco2_per_kg, slag_kgco2_per_kg):
    # TODO: return the slag mass in kilograms, as a float.
    ...


# ---------------------------------------------------------------------------
# Provided, do not edit. This runs your functions so you can try them by hand.
# The grader never runs it; it calls your functions directly.
# ---------------------------------------------------------------------------

def main():
    binder_target_tonnes = float(input("Binder mass target (tonnes): "))
    co2_target_kgco2_per_tonne = float(input("CO2 target (kg CO2 per tonne): "))
    flyash_kgco2_per_kg = float(input("Fly ash intensity (kg CO2 per kg): "))
    slag_kgco2_per_kg = float(input("Slag intensity (kg CO2 per kg): "))

    print("fly ash kg:", fly_ash_mass_kg(
        binder_target_tonnes, co2_target_kgco2_per_tonne,
        flyash_kgco2_per_kg, slag_kgco2_per_kg))
    print("slag kg:", slag_mass_kg(
        binder_target_tonnes, co2_target_kgco2_per_tonne,
        flyash_kgco2_per_kg, slag_kgco2_per_kg))


if __name__ == "__main__":
    main()
