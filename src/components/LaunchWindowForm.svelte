<script>
  // Supply-run break-even: the exact break-even point is
  //   r = deficit_kg / (delivered_kg - cost_kg)
  // and the console needs the number of complete runs, the smallest whole
  // number that reaches the deficit (round the exact point up).
  let deficit_kg = 1000;
  let delivered_kg = 200;
  let cost_kg = 50;

  $: net_per_run = delivered_kg - cost_kg;
  $: exact = net_per_run > 0 ? deficit_kg / net_per_run : 0;
  $: runs = net_per_run > 0 ? Math.ceil(exact) : 0;
  $: surplus = net_per_run > 0 ? runs * net_per_run - deficit_kg : 0;

  const round = (x) => Math.round(x * 1000) / 1000;
</script>

<div class="not-content flex flex-col gap-4 print:hidden">
  <div class="w-full sm:w-md rounded border p-4 space-y-4 mx-auto my-4">
    <h3 class="text-xl">Supply-run break-even calculator</h3>
    <p class="text-sm">
      Change any value to see how many complete supply runs the crew must fly.
      The calculator uses the same break-even relation you are implementing,
      then rounds up to whole runs.
    </p>
    <div class="grid grid-cols-2 gap-3">
      <div>
        <label class="font-bold text-sm block mb-1 min-h-[2.5rem]" for="deficit_kg">Mass deficit (deficit_kg)</label>
        <input class="border rounded p-2 w-full" type="number" id="deficit_kg" bind:value={deficit_kg} step="10" />
      </div>
      <div>
        <label class="font-bold text-sm block mb-1 min-h-[2.5rem]" for="delivered_kg">Delivered per run (delivered_kg)</label>
        <input class="border rounded p-2 w-full" type="number" id="delivered_kg" bind:value={delivered_kg} step="10" />
      </div>
      <div>
        <label class="font-bold text-sm block mb-1 min-h-[2.5rem]" for="cost_kg">Launch cost per run (cost_kg)</label>
        <input class="border rounded p-2 w-full" type="number" id="cost_kg" bind:value={cost_kg} step="10" />
      </div>
    </div>

    {#if net_per_run <= 0}
      <p class="text-red-600">
        Each run delivers no more than it spends to launch (delivered_kg is not
        greater than cost_kg), so the runs never erase the deficit. Choose a
        delivered mass greater than the launch cost.
      </p>
    {:else}
      <div class="rounded p-4 bg-gray-50 dark:bg-gray-800 space-y-1">
        <div>Net per run: {net_per_run} kg</div>
        <div>Exact break-even point: {round(exact)} runs</div>
        <div>Complete supply runs: {runs}</div>
        <div class="text-sm opacity-70">
          Surplus after {runs} runs: {surplus} kg
        </div>
      </div>
    {/if}
  </div>
</div>
