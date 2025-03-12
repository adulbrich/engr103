<script>
  let present_value = 1000;
  let annual_interest_rate = 7;
  let number_of_periods = 10;
  let recurring_payments = 100;
  let future_value = 10000;
  let payment_type = "start";
  let annual_inflation_rate = 0;
  let compounding_period = "year"; // "year" or "month"

  $: rate_factor = compounding_period === "month" ? 1 / 12 : 1;
  $: period_factor = compounding_period === "month" ? 12 : 1;
  $: real_annual_interest_rate =
      ((1 + annual_interest_rate / 100) / (1 + annual_inflation_rate / 100) -
        1) *
      rate_factor;
  $: periods_adjusted = number_of_periods * period_factor;

  $: extra_period =
    payment_type === "start" ? 1 + real_annual_interest_rate : 1;

  $: fv =
    present_value * (1 + real_annual_interest_rate) ** periods_adjusted +
    (recurring_payments *
      extra_period *
      ((1 + real_annual_interest_rate) ** periods_adjusted - 1)) /
      real_annual_interest_rate;

  $: pmt =
    (future_value -
      present_value * (1 + real_annual_interest_rate) ** periods_adjusted) /
    ((extra_period *
      ((1 + real_annual_interest_rate) ** periods_adjusted - 1)) /
      real_annual_interest_rate);
  $: pv =
    (future_value -
      (recurring_payments *
        extra_period *
        ((1 + real_annual_interest_rate) ** periods_adjusted - 1)) /
        real_annual_interest_rate) /
    (1 + real_annual_interest_rate) ** periods_adjusted;

  $: fv_currency = new Intl.NumberFormat(`en-US`, {
    currency: `USD`,
    style: "currency",
  }).format(fv);
  $: pmt_currency = new Intl.NumberFormat(`en-US`, {
    currency: `USD`,
    style: "currency",
  }).format(pmt);
  $: pv_currency = new Intl.NumberFormat(`en-US`, {
    currency: `USD`,
    style: "currency",
  }).format(pv);

  let activeTab = "future"; // Options: 'future', 'payments', 'present'
</script>

<div class="w-full max-w-2xl mx-auto p-4 pt-0 mt-4 rounded border print:hidden dark:bg-black dark:text-white">
  <!-- Tab Navigation -->
  <div class="flex w-full border-b border-white dark:border-black mb-4">
    <button
      class={`flex-1 px-4 py-2 mt-4 text-center bg-stone-100 dark:bg-stone-800 ${activeTab === "future" ? "border-b-2 border-orange-500 text-orange-500" : ""}`}
      on:click={() => (activeTab = "future")}
    >
      Future Value
    </button>
    <button
      class={`flex-1 px-4 py-2 mt-4 text-center bg-stone-100 dark:bg-stone-800 ${activeTab === "payments" ? "border-b-2 border-orange-500 text-orange-600" : ""}`}
      on:click={() => (activeTab = "payments")}
    >
      Recurring Payments
    </button>
    <button
      class={`flex-1 px-4 py-2 mt-4 text-center bg-stone-100 dark:bg-stone-800 ${activeTab === "present" ? "border-b-2 border-orange-500 text-orange-600" : ""}`}
      on:click={() => (activeTab = "present")}
    >
      Present Value
    </button>
  </div>

  <!-- Tab Content -->
  <div class="space-y-4">
    <!-- Common Inputs -->
    <div class="grid grid-cols-1 gap-4">
      <div>
        <label class="block text-sm font-medium"
          >Payment Timing
          <div class="mt-1 space-x-4">
            <label class="inline-flex items-center">
              <input
                type="radio"
                bind:group={payment_type}
                value="start"
                class="form-radio"
              />
              <span class="ml-2">Start of Period</span>
            </label>
            <label class="inline-flex items-center">
              <input
                type="radio"
                bind:group={payment_type}
                value="end"
                class="form-radio"
              />
              <span class="ml-2">End of Period</span>
            </label>
          </div></label
        >
      </div>

      <div class="mt-4 space-y-2">
        <label class="block text-sm font-medium"
          >Compounding Period
          <div class="mt-1 space-x-4">
            <label class="inline-flex items-center">
              <input
                type="radio"
                bind:group={compounding_period}
                value="year"
                class="form-radio"
              />
              <span class="ml-2">Yearly</span>
            </label>
            <label class="inline-flex items-center">
              <input
                type="radio"
                bind:group={compounding_period}
                value="month"
                class="form-radio"
              />
              <span class="ml-2">Monthly</span>
            </label>
          </div>
        </label>
      </div>

      <div>
        <label class="block text-sm font-medium"
          >Annual Return Rate (%)
          <input
            type="number"
            bind:value={annual_interest_rate}
            class="mt-1 block w-full rounded-md border-stone-300 shadow-sm focus-visible:outline-orange-500"
            min="0"
            max="100"
            step="0.5"
          /></label
        >
      </div>

      <div>
        <label class="block text-sm font-medium"
          >Annual Inflation Rate (%) - Optional
          <input
            type="number"
            bind:value={annual_inflation_rate}
            class="mt-1 block w-full rounded-md border-stone-300 shadow-sm focus-visible:outline-orange-500"
            min="0"
            max="100"
            step="0.5"
          /></label
        >
      </div>

      <div>
        <label class="block text-sm font-medium"
          >Number of Periods
          <input
            type="number"
            bind:value={number_of_periods}
            class="mt-1 block w-full rounded-md border-stone-300 shadow-sm focus-visible:outline-orange-500"
          /></label
        >
      </div>
    </div>

    {#if activeTab === "future"}
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium"
            >Present Value
            <input
              type="number"
              bind:value={present_value}
              class="mt-1 block w-full rounded-md border-stone-300 shadow-sm focus-visible:outline-orange-500"
            /></label
          >
        </div>
        <div>
          <label class="block text-sm font-medium"
            >Periodic Deposits (per period)
            <input
              type="number"
              bind:value={recurring_payments}
              class="mt-1 block w-full rounded-md border-stone-300 shadow-sm focus-visible:outline-orange-500"
            /></label
          >
        </div>
        {#if fv}
          <div class="p-4 bg-stone-100 dark:bg-stone-800 rounded-md">
            <p class="font-medium">Future Value: {fv_currency}</p>
          </div>
        {/if}
      </div>
    {:else if activeTab === "payments"}
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium"
            >Present Value
            <input
              type="number"
              bind:value={present_value}
              class="mt-1 block w-full rounded-md border-stone-300 shadow-sm focus-visible:outline-orange-500"
            /></label
          >
        </div>
        <div>
          <label class="block text-sm font-medium"
            >Future Value
            <input
              type="number"
              bind:value={future_value}
              class="mt-1 block w-full rounded-md border-stone-300 shadow-sm focus-visible:outline-orange-500"
            /></label
          >
        </div>
        {#if pmt}
          <div class="p-4 bg-stone-100 dark:bg-stone-800 rounded-md">
            <p class="font-medium">Required Payment: {pmt_currency}</p>
          </div>
        {/if}
      </div>
    {:else}
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium"
            >Future Value
            <input
              type="number"
              bind:value={future_value}
              class="mt-1 block w-full rounded-md border-stone-300 shadow-sm focus-visible:outline-orange-500"
            /></label
          >
        </div>
        <div>
          <label class="block text-sm font-medium"
            >Monthly Payment
            <input
              type="number"
              bind:value={recurring_payments}
              class="mt-1 block w-full rounded-md border-stone-300 shadow-sm focus-visible:outline-orange-500"
            /></label
          >
        </div>
        {#if pv}
          <div class="p-4 bg-stone-100 dark:bg-stone-800 rounded-md">
            <p class="font-medium">Present Value: {pv_currency}</p>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>
