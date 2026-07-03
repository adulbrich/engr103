<!--
  Binary / bits visualizer.
  Lets a reader type a non-negative whole number and see, reactively, its
  binary digits grouped into bytes, how a fixed bit-width (8/16/32) either
  holds that value or cannot, and (optionally) the ASCII character for
  values 0-127. Pure client-side, no network, no external libraries.
-->
<script>
  const WIDTHS = [8, 16, 32];

  let rawValue = $state(65);
  let width = $state(8);

  function sanitize(n) {
    if (!Number.isFinite(n)) return 0;
    return Math.max(0, Math.floor(n));
  }

  const value = $derived(sanitize(rawValue));

  function bitsNeeded(n) {
    return n <= 0 ? 1 : n.toString(2).length;
  }

  const minBits = $derived(bitsNeeded(value));
  const overflow = $derived(minBits > width);

  // Total bits to display: the selected width normally, or enough extra
  // whole bytes to show the number when it does not fit that width.
  const totalBits = $derived(overflow ? Math.ceil(minBits / 8) * 8 : width);

  const binaryDigits = $derived(value.toString(2).padStart(totalBits, '0'));

  // How many of the leading (most-significant) bytes are "overflow" bytes,
  // i.e. bits that a fixed `width`-bit integer would not have room for.
  const overflowByteCount = $derived(overflow ? (totalBits - width) / 8 : 0);

  const bytes = $derived.by(() => {
    const groups = [];
    for (let i = 0; i < binaryDigits.length; i += 8) {
      groups.push({
        bits: binaryDigits.slice(i, i + 8),
        overflow: i / 8 < overflowByteCount,
      });
    }
    return groups;
  });

  const maxForWidth = $derived(2 ** width - 1);

  const ascii = $derived.by(() => {
    if (value > 127) return null;
    const printable = value >= 32 && value < 127;
    return {
      printable,
      char: printable ? String.fromCharCode(value) : null,
    };
  });
</script>

<div>
  <div class="hidden print:block text-sm italic text-[var(--sl-color-gray-4)]">
    Interactive binary visualizer example is not available when printing. Please view this page online.
  </div>

  <div class="print:hidden rounded-lg border border-[var(--sl-color-hairline)] bg-[var(--sl-color-bg-nav)] p-4">
    <h3 class="m-0 mb-3 text-base font-semibold text-[var(--sl-color-white)]">Binary / bits visualizer</h3>

    <div class="flex flex-wrap items-end gap-6">
      <div class="flex flex-col gap-1">
        <label for="bv-decimal" class="text-xs font-semibold uppercase tracking-wide text-[var(--sl-color-gray-3)]">
          Decimal value
        </label>
        <input
          id="bv-decimal"
          type="number"
          min="0"
          step="1"
          inputmode="numeric"
          bind:value={rawValue}
          class="w-32 rounded-md border border-[var(--sl-color-gray-5)] bg-[var(--sl-color-bg)] px-2 py-1.5 font-mono text-[var(--sl-color-white)]"
        />
      </div>

      <fieldset class="m-0 flex flex-col gap-1 border-0 p-0">
        <legend class="mb-1 text-xs font-semibold uppercase tracking-wide text-[var(--sl-color-gray-3)]">
          Width (bits)
        </legend>
        <div class="flex gap-2">
          {#each WIDTHS as w (w)}
            <button
              type="button"
              aria-pressed={width === w}
              onclick={() => (width = w)}
              class={
                'rounded-md border px-3 py-1.5 text-sm font-mono ' +
                (width === w
                  ? 'border-[var(--sl-color-accent)] bg-[var(--sl-color-accent-low)] text-[var(--sl-color-accent-high)]'
                  : 'border-[var(--sl-color-gray-5)] text-[var(--sl-color-gray-2)]')
              }
            >
              {w}-bit
            </button>
          {/each}
        </div>
      </fieldset>
    </div>

    <p class="mt-2 text-sm text-[var(--sl-color-gray-2)]">
      Decimal: <span class="font-mono">{value}</span>.
      A {width}-bit integer can hold values from 0 to <span class="font-mono">{maxForWidth}</span>.
    </p>

    <div class="mt-4">
      <div class="mb-1 text-xs font-semibold uppercase tracking-wide text-[var(--sl-color-gray-3)]">
        Binary, grouped into bytes
      </div>
      <div class="flex flex-wrap gap-2" role="group" aria-label="Binary representation grouped into bytes">
        {#each bytes as byte, i (i)}
          <span
            class={
              'rounded px-2 py-1 font-mono text-lg tracking-widest ' +
              (byte.overflow
                ? 'bg-[var(--sl-color-red-low)] text-[var(--sl-color-red-high)]'
                : 'bg-[var(--sl-color-bg-inline-code)] text-[var(--sl-color-white)]')
            }
          >{byte.bits}</span>
        {/each}
      </div>
    </div>

    {#if overflow}
      <p class="mt-3 rounded-md border border-[var(--sl-color-red)] bg-[var(--sl-color-red-low)] px-3 py-2 text-sm text-[var(--sl-color-red-high)]">
        This number needs more than {width} bits (it needs at least {minBits} bits), so a fixed {width}-bit integer cannot hold it.
        The bytes shaded above are the extra bits that do not fit.
      </p>
    {/if}

    {#if ascii}
      <div class="mt-4 rounded-md border border-[var(--sl-color-gray-5)] px-3 py-2 text-sm text-[var(--sl-color-gray-2)]">
        <span class="text-xs font-semibold uppercase tracking-wide text-[var(--sl-color-gray-3)]">Optional: ASCII</span>
        <p class="m-0 mt-1">
          {#if ascii.printable}
            The value {value} also maps to the ASCII character <span class="font-mono text-base">'{ascii.char}'</span>.
          {:else}
            The value {value} is in the ASCII range (0-127), but it is a non-printable control character.
          {/if}
        </p>
      </div>
    {/if}
  </div>
</div>
