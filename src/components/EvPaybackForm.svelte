<script>
  // EV carbon payback: the payback distance is the extra manufacturing
  // footprint divided by how many fewer grams of CO2e the EV emits per
  // kilometre than the gasoline vehicle it replaces, then split into whole
  // years of typical driving plus the leftover kilometres in the final year.
  //   saving_per_km_gco2 = gasoline_per_km_gco2 - ev_per_km_gco2
  //   payback_km = extra_manufacturing_gco2 // saving_per_km_gco2
  //   whole_years = payback_km // annual_km
  //   leftover_km = payback_km % annual_km
  let extra_manufacturing_gco2 = $state(4800000);
  let gasoline_per_km_gco2 = $state(190);
  let ev_per_km_gco2 = $state(55);
  let annual_km = $state(15000);

  let saving_per_km_gco2 = $derived(gasoline_per_km_gco2 - ev_per_km_gco2);
  let payback_km = $derived(
    saving_per_km_gco2 > 0 ? Math.floor(extra_manufacturing_gco2 / saving_per_km_gco2) : 0
  );
  let whole_years = $derived(annual_km > 0 ? Math.floor(payback_km / annual_km) : 0);
  let leftover_km = $derived(annual_km > 0 ? payback_km % annual_km : 0);
</script>

<div class="not-content flex flex-col gap-4 print:hidden">
  <div class="w-full sm:w-md rounded border p-4 space-y-4 mx-auto my-4">
    <h3 class="text-xl">EV payback calculator</h3>
    <p class="text-sm">
      Change any value to see how far an EV must be driven before its extra
      manufacturing footprint pays back. The calculator uses the same
      computation you are implementing. The manufacturing footprint is in
      grams of CO2e, the emissions figures are in grams of CO2e per
      kilometre, and the distances are in kilometres.
    </p>
    <div class="grid grid-cols-2 gap-3">
      <div>
        <label class="font-bold text-sm block mb-1 min-h-[2.5rem]" for="extra_manufacturing_gco2">
          Extra manufacturing (extra_manufacturing_gco2)
        </label>
        <input
          class="border rounded p-2 w-full"
          type="number"
          id="extra_manufacturing_gco2"
          bind:value={extra_manufacturing_gco2}
          step="10000"
        />
      </div>
      <div>
        <label class="font-bold text-sm block mb-1 min-h-[2.5rem]" for="annual_km">
          Annual distance (annual_km)
        </label>
        <input
          class="border rounded p-2 w-full"
          type="number"
          id="annual_km"
          bind:value={annual_km}
          step="500"
        />
      </div>
      <div>
        <label class="font-bold text-sm block mb-1 min-h-[2.5rem]" for="gasoline_per_km_gco2">
          Gasoline emissions (gasoline_per_km_gco2)
        </label>
        <input
          class="border rounded p-2 w-full"
          type="number"
          id="gasoline_per_km_gco2"
          bind:value={gasoline_per_km_gco2}
          step="1"
        />
      </div>
      <div>
        <label class="font-bold text-sm block mb-1 min-h-[2.5rem]" for="ev_per_km_gco2">
          EV emissions (ev_per_km_gco2)
        </label>
        <input
          class="border rounded p-2 w-full"
          type="number"
          id="ev_per_km_gco2"
          bind:value={ev_per_km_gco2}
          step="1"
        />
      </div>
    </div>

    {#if saving_per_km_gco2 <= 0}
      <p class="text-red-600">
        The EV's per-kilometre figure is not less than the gasoline vehicle's
        (ev_per_km_gco2 is not less than gasoline_per_km_gco2), so driving it
        never pays back the extra manufacturing footprint. Choose an EV figure
        that is lower.
      </p>
    {:else if annual_km <= 0}
      <p class="text-red-600">
        The typical annual distance must be greater than zero.
      </p>
    {:else}
      <div class="rounded p-4 bg-gray-50 dark:bg-gray-800 space-y-1">
        <div>Saving per kilometre: {saving_per_km_gco2} gCO2e/km</div>
        <div>Payback distance: {payback_km} km</div>
        <div>Whole years: {whole_years}</div>
        <div class="text-sm opacity-70">Leftover kilometres in the final year: {leftover_km}</div>
      </div>
    {/if}
  </div>
</div>
