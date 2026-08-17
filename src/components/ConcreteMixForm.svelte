<script>
  // Low-carbon concrete mix: solve x + y = M and a*x + b*y = C for the two
  // material masses, where M is the binder target (tonnes -> kg) and C is
  // the CO2 target (a rate in kgCO2/tonne -> an absolute kgCO2 total).
  //   binder_target_kg = binder_target_tonnes * 1000
  //   co2_target_kgco2 = co2_target_kgco2_per_tonne * binder_target_tonnes
  //   x (fly ash) = (co2_target_kgco2 - slag * binder_target_kg) / (flyash - slag)
  //   y (slag) = binder_target_kg - x
  let binder_target_tonnes = $state(8);
  let co2_target_kgco2_per_tonne = $state(203.125);
  let flyash_kgco2_per_kg = $state(0.125);
  let slag_kgco2_per_kg = $state(0.25);

  let binder_target_kg = $derived(binder_target_tonnes * 1000);
  let co2_target_kgco2 = $derived(co2_target_kgco2_per_tonne * binder_target_tonnes);
  let delta = $derived(flyash_kgco2_per_kg - slag_kgco2_per_kg);
  let fly_ash_mass_kg = $derived(
    delta !== 0 ? (co2_target_kgco2 - slag_kgco2_per_kg * binder_target_kg) / delta : 0
  );
  let slag_mass_kg = $derived(binder_target_kg - fly_ash_mass_kg);

  const round = (x) => Math.round(x * 1000) / 1000;
</script>

<div class="not-content flex flex-col gap-4 print:hidden">
  <div class="w-full sm:w-md rounded border p-4 space-y-4 mx-auto my-4">
    <h3 class="text-xl">Concrete mix calculator</h3>
    <p class="text-sm">
      Change any value to see how the two material masses respond. The
      calculator uses the same equations you are implementing. The binder
      target is in tonnes, the CO2 target is a rate in kilograms of CO2 per
      tonne, and the two intensities are in kilograms of CO2 per kilogram of
      material.
    </p>
    <div class="grid grid-cols-2 gap-3">
      <div>
        <label class="font-bold text-sm block mb-1 min-h-[2.5rem]" for="binder_target_tonnes">
          Binder target (binder_target_tonnes)
        </label>
        <input
          class="border rounded p-2 w-full"
          type="number"
          id="binder_target_tonnes"
          bind:value={binder_target_tonnes}
          step="0.5"
        />
      </div>
      <div>
        <label class="font-bold text-sm block mb-1 min-h-[2.5rem]" for="co2_target_kgco2_per_tonne">
          CO2 target rate (co2_target_kgco2_per_tonne)
        </label>
        <input
          class="border rounded p-2 w-full"
          type="number"
          id="co2_target_kgco2_per_tonne"
          bind:value={co2_target_kgco2_per_tonne}
          step="0.5"
        />
      </div>
      <div>
        <label class="font-bold text-sm block mb-1 min-h-[2.5rem]" for="flyash_kgco2_per_kg">
          Fly ash intensity (flyash_kgco2_per_kg)
        </label>
        <input
          class="border rounded p-2 w-full"
          type="number"
          id="flyash_kgco2_per_kg"
          bind:value={flyash_kgco2_per_kg}
          step="0.01"
        />
      </div>
      <div>
        <label class="font-bold text-sm block mb-1 min-h-[2.5rem]" for="slag_kgco2_per_kg">
          Slag intensity (slag_kgco2_per_kg)
        </label>
        <input
          class="border rounded p-2 w-full"
          type="number"
          id="slag_kgco2_per_kg"
          bind:value={slag_kgco2_per_kg}
          step="0.01"
        />
      </div>
    </div>

    {#if delta === 0}
      <p class="text-red-600">
        The two materials have identical CO2 intensities (flyash_kgco2_per_kg
        equals slag_kgco2_per_kg), so there is no single solution. Choose
        intensities that differ.
      </p>
    {:else}
      <div class="rounded p-4 bg-gray-50 dark:bg-gray-800 space-y-1">
        <div>Fly ash mass (x): {round(fly_ash_mass_kg)} kg</div>
        <div>Slag mass (y): {round(slag_mass_kg)} kg</div>
        <div class="text-sm opacity-70">
          Check: mass = {round(fly_ash_mass_kg + slag_mass_kg)} kg, CO2 = {round(
            flyash_kgco2_per_kg * fly_ash_mass_kg + slag_kgco2_per_kg * slag_mass_kg
          )} kg
        </div>
      </div>
    {/if}
  </div>
</div>
