<script>
  // Two-stage injection burn: find the two burn times t1, t2 that satisfy
  //   v1*t1 + v2*t2 = total_dv   (speeds add to the target)
  //   f1*t1 + f2*t2 = fuel_budget (fuels add to the budget)
  let v1 = 5;
  let v2 = 8;
  let f1 = 3;
  let f2 = 2;
  let total_dv = 1240;
  let fuel_budget = 520;

  $: det = v1 * f2 - v2 * f1;
  $: t1 = det !== 0 ? (total_dv * f2 - v2 * fuel_budget) / det : 0;
  $: t2 = det !== 0 ? (v1 * fuel_budget - total_dv * f1) / det : 0;

  const round = (x) => Math.round(x * 1000) / 1000;
</script>

<div class="not-content flex flex-col gap-4 print:hidden">
  <div class="w-full sm:w-md rounded border p-4 space-y-4 mx-auto my-4">
    <h3 class="text-xl">Two-stage burn calculator</h3>
    <p class="text-sm">
      Change any value to see how the two burn times respond. The calculator
      uses the same equations you are implementing.
    </p>
    <div class="grid grid-cols-2 gap-3">
      <div>
        <label class="font-bold text-sm block mb-1" for="v1">Stage 1 speed rate (v1)</label>
        <input class="border rounded p-2 w-full" type="number" id="v1" bind:value={v1} step="0.1" />
      </div>
      <div>
        <label class="font-bold text-sm block mb-1" for="v2">Stage 2 speed rate (v2)</label>
        <input class="border rounded p-2 w-full" type="number" id="v2" bind:value={v2} step="0.1" />
      </div>
      <div>
        <label class="font-bold text-sm block mb-1" for="f1">Stage 1 fuel rate (f1)</label>
        <input class="border rounded p-2 w-full" type="number" id="f1" bind:value={f1} step="0.1" />
      </div>
      <div>
        <label class="font-bold text-sm block mb-1" for="f2">Stage 2 fuel rate (f2)</label>
        <input class="border rounded p-2 w-full" type="number" id="f2" bind:value={f2} step="0.1" />
      </div>
      <div>
        <label class="font-bold text-sm block mb-1" for="total_dv">Total speed target (total_dv)</label>
        <input class="border rounded p-2 w-full" type="number" id="total_dv" bind:value={total_dv} step="1" />
      </div>
      <div>
        <label class="font-bold text-sm block mb-1" for="fuel_budget">Fuel budget (fuel_budget)</label>
        <input class="border rounded p-2 w-full" type="number" id="fuel_budget" bind:value={fuel_budget} step="1" />
      </div>
    </div>

    {#if det === 0}
      <p class="text-red-600">
        The two stages have identical performance (v1 f2 equals v2 f1), so there
        is no single solution. Choose stages that differ.
      </p>
    {:else}
      <div class="rounded p-4 bg-gray-50 dark:bg-gray-800 space-y-1">
        <div>Stage 1 burn time (t1): {round(t1)} s</div>
        <div>Stage 2 burn time (t2): {round(t2)} s</div>
        <div class="text-sm opacity-70">
          Check: speed = {round(v1 * t1 + v2 * t2)}, fuel = {round(f1 * t1 + f2 * t2)}
        </div>
      </div>
    {/if}
  </div>
</div>
