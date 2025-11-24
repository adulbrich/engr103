<script>
  // Problem 1
  let r_p = null; // revenue per product
  let c_p = null; // manufacturing cost per product
  let b = null; // base cost of product (before manufacturing)
  let problemOneError = "";

  $: {
    if (r_p <= c_p) {
      problemOneError =
        "Revenue per product must be greater than cost per product.";
    } else {
      problemOneError = "";
    }
  }

  $: N = r_p > c_p ? Math.ceil((b / (r_p - c_p))) : 0;

  // Problem 2
  let n1 = 0.07;
  let n2 = 0.03;
  let n3 = 0.05;
  let p1 = 0.01;
  let p2 = 0.09;
  let p3 = 0.04;
  let k1 = 0.02;
  let k2 = 0.02;
  let k3 = 0.12;
  let problemTwoError = "";

  $: M1_denominator =
    k3 -
    n3 +
    n1 -
    k1 +
    (n3 - n2 + k2 - k3) * ((n1 - n3 + p3 - p1) / (n2 - n3 + p3 - p2));
  $: M1 =
    M1_denominator !== 0
      ? (k3 - n3 + (n3 - n2 + k2 - k3) * ((p3 - n3) / (n2 - n3 + p3 - p2))) /
        M1_denominator
      : 0;
  $: M2_denominator = n2 - n3 + p3 - p2;
  $: M2 =
    M2_denominator !== 0
      ? (p3 - n3 - M1 * (n1 - n3 + p3 - p1)) / M2_denominator
      : 0;
  $: M3 = 1 - M1 - M2;
  $: nitrogen_proportion = M1 * n1 + M2 * n2 + M3 * n3;
  $: phosphorus_proportion = M1 * p1 + M2 * p2 + M3 * p3;
  $: potassium_proportion = M1 * k1 + M2 * k2 + M3 * k3;

  $: {
    if (M1 < 0 || M2 < 0 || M3 < 0) {
      problemTwoError =
        "Impossible to create a balanced fertilizer using the given inputs.";
    } else if (M1_denominator === 0 || M2_denominator === 0) {
      problemTwoError = "Denominator(s) cannot be zero.";
    } else {
      problemTwoError = "";
    }
  }
</script>

<div class="flex flex-col gap-4 print:hidden">
  <div class="w-72 sm:w-96 rounded border p-4 space-y-4 mx-auto my-4">
    <h3 class="text-xl">Problem 1 Simulator</h3>
    <div>
      <label class="font-bold text-sm" for="b"
        >Base cost before manufacturing (b)</label
      >
      <input
        class="w-full border rounded p-2"
        type="number"
        id="b"
        bind:value={b}
        placeholder="Enter base cost"
        step="0.01"
      />
    </div>
    <div>
      <label class="font-bold text-sm" for="r_p"
        >Revenue per product (r_p)</label
      >
      <input
        class="w-full border rounded p-2"
        type="number"
        id="r_p"
        bind:value={r_p}
        placeholder="Enter revenue per product"
        step="0.01"
      />
    </div>
    <div>
      <label class="font-bold text-sm" for="c_p"
        >Manufacturing cost per product (c_p)</label
      >
      <input
        class="w-full border rounded p-2"
        type="number"
        id="c_p"
        bind:value={c_p}
        placeholder="Enter cost per product"
        step="0.01"
      />
    </div>

    {#if problemOneError}
      <p class="text-red-600">{problemOneError}</p>
    {:else}
      <div class="rounded p-4 bg-gray-50 dark:bg-gray-800">
        Number of products needed (N): {N}
      </div>
    {/if}
  </div>

  <div class="w-72 sm:w-96 rounded border p-4 space-y-4 mx-auto my-4">
    <h3 class="text-xl">Problem 2 Simulator</h3>
    <div class="flex flex-row gap-4">
      <div class="flex flex-col justify-between items-center">
        <div class="flex flex-col w-24">
          <label class="font-bold text-sm" for="n1">n1</label>
          <input
            class="border rounded p-2"
            type="number"
            id="n1"
            bind:value={n1}
            step="0.01"
          />
        </div>
        <div class="flex flex-col w-24">
          <label class="font-bold text-sm" for="n2">n2</label>
          <input
            class="border rounded p-2"
            type="number"
            id="n2"
            bind:value={n2}
            step="0.01"
          />
        </div>
        <div class="flex flex-col w-24">
          <label class="font-bold text-sm" for="n3">n3</label>
          <input
            class="border rounded p-2"
            type="number"
            id="n3"
            bind:value={n3}
            step="0.01"
          />
        </div>
        <div class="flex flex-col w-24">
          <label class="font-bold text-sm" for="p1">p1</label>
          <input
            class="border rounded p-2"
            type="number"
            id="p1"
            bind:value={p1}
            step="0.01"
          />
        </div>
        <div class="flex flex-col w-24">
          <label class="font-bold text-sm" for="p2">p2</label>
          <input
            class="border rounded p-2"
            type="number"
            id="p2"
            bind:value={p2}
            step="0.01"
          />
        </div>
        <div class="flex flex-col w-24">
          <label class="font-bold text-sm" for="p3">p3</label>
          <input
            class="border rounded p-2"
            type="number"
            id="p3"
            bind:value={p3}
            step="0.01"
          />
        </div>
        <div class="flex flex-col w-24">
          <label class="font-bold text-sm" for="k1">k1</label>
          <input
            class="border rounded p-2"
            type="number"
            id="k1"
            bind:value={k1}
            step="0.01"
          />
        </div>
        <div class="flex flex-col w-24">
          <label class="font-bold text-sm" for="k2">k2</label>
          <input
            class="border rounded p-2"
            type="number"
            id="k2"
            bind:value={k2}
            step="0.01"
          />
        </div>
        <div class="flex flex-col w-24">
          <label class="font-bold text-sm m-0" for="k3">k3</label>
          <input
            class="border rounded p-2"
            type="number"
            id="k3"
            bind:value={k3}
            step="0.01"
          />
        </div>
      </div>
      <div class="w-48 rounded p-4 bg-gray-50 dark:bg-gray-800 mx-auto flex flex-col">
        {#if problemTwoError}
          <p class="text-red-600">{problemTwoError}</p>
        {:else}
          <div class="flex flex-row justify-between">
            <span>M1</span> <span>{M1.toFixed(3)}</span>
          </div>
          <div class="flex flex-row justify-between">
            <span>M2</span> <span>{M2.toFixed(3)}</span>
          </div>
          <div class="flex flex-row justify-between">
            <span>M3</span> <span>{M3.toFixed(3)}</span>
          </div>
          <div class="flex flex-row justify-between">
            <span>Nitrogen Proportion</span>
            <span>{nitrogen_proportion.toFixed(3)}</span>
          </div>
          <div class="flex flex-row justify-between">
            <span>Phosphorus Proportion</span>
            <span>{phosphorus_proportion.toFixed(3)}</span>
          </div>
          <div class="flex flex-row justify-between">
            <span>Potassium Proportion</span>
            <span>{potassium_proportion.toFixed(3)}</span>
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>
