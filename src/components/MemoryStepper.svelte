<!--
  Dual-language memory stepper.
  Walks a reader through a pre-authored sequence of steps that visualize how
  names point at values: Python aliasing (two names, one shared object) versus
  Rust ownership, moves, and borrows. This component does NOT parse or execute
  code; it renders an authored trace passed in as a prop, which is robust and
  reliable. Pure client-side, no network, no external libraries.
-->
<script>
  let {
    title = '',
    code = '',
    steps = [],
  } = $props();

  const codeLines = $derived(String(code).replace(/\r\n/g, '\n').replace(/\r/g, '\n').split('\n'));

  const stepCount = $derived(Array.isArray(steps) ? steps.length : 0);

  let stepIndex = $state(0);

  // Keep the index inside the valid range even if `steps` changes length.
  const safeIndex = $derived(
    stepCount === 0 ? 0 : Math.min(Math.max(stepIndex, 0), stepCount - 1)
  );

  const currentStep = $derived(stepCount === 0 ? null : steps[safeIndex] ?? null);

  const currentNames = $derived(Array.isArray(currentStep?.names) ? currentStep.names : []);
  const currentObjects = $derived(Array.isArray(currentStep?.objects) ? currentStep.objects : []);

  // 1-based highlighted line for the current step.
  const activeLine = $derived(currentStep && typeof currentStep.line === 'number' ? currentStep.line : -1);

  function goPrev() {
    if (safeIndex > 0) stepIndex = safeIndex - 1;
  }

  function goNext() {
    if (safeIndex < stepCount - 1) stepIndex = safeIndex + 1;
  }

  function nameState(nm) {
    return nm && (nm.state === 'moved' || nm.state === 'borrow') ? nm.state : 'owned';
  }
</script>

<div>
  <div class="hidden print:block text-sm italic text-[var(--sl-color-gray-4)]">
    Interactive memory stepper example is not available when printing. Please view this page online.
  </div>

  <div class="print:hidden rounded-lg border border-[var(--sl-color-hairline)] bg-[var(--sl-color-bg-nav)] p-4">
    {#if title}
      <h3 class="m-0 mb-3 text-base font-semibold text-[var(--sl-color-white)]">{title}</h3>
    {/if}

    <div class="grid gap-4 md:grid-cols-3">
      <!-- Column 1: code with the current line highlighted -->
      <div class="min-w-0">
        <div class="mb-1 text-xs font-semibold uppercase tracking-wide text-[var(--sl-color-gray-3)]">
          Code
        </div>
        <div
          class="overflow-auto rounded-md border border-[var(--sl-color-gray-5)] bg-[var(--sl-color-bg-inline-code)] font-mono text-sm"
          role="group"
          aria-label="Program code, current line highlighted"
        >
          {#each codeLines as line, i (i)}
            {@const isActive = activeLine === i + 1}
            <div
              class={[
                'flex gap-3 px-3 py-0.5 leading-6',
                isActive
                  ? 'bg-[var(--sl-color-accent-low)] text-[var(--sl-color-white)]'
                  : 'text-[var(--sl-color-gray-2)]',
              ]}
              aria-current={isActive ? 'step' : undefined}
            >
              <span class="w-6 shrink-0 select-none text-right text-[var(--sl-color-gray-4)]" aria-hidden="true">
                {i + 1}
              </span>
              <span class="min-w-0 flex-1 whitespace-pre">{line}{#if isActive}<span class="sr-only"> (current line)</span>{/if}</span>
            </div>
          {/each}
        </div>
      </div>

      <!-- Column 2: names -->
      <div class="min-w-0">
        <div class="mb-1 text-xs font-semibold uppercase tracking-wide text-[var(--sl-color-gray-3)]">
          Names
        </div>
        <div class="flex flex-col gap-2">
          {#each currentNames as nm (nm.name)}
            {@const state = nameState(nm)}
            <div
              class={[
                'rounded-md border px-3 py-2 font-mono text-sm',
                state === 'moved' && 'border-[var(--sl-color-gray-5)] bg-[var(--sl-color-gray-6)] text-[var(--sl-color-gray-4)]',
                state === 'borrow' && 'border-[var(--sl-color-accent)] bg-[var(--sl-color-accent-low)] text-[var(--sl-color-accent-high)]',
                state === 'owned' && 'border-[var(--sl-color-gray-5)] bg-[var(--sl-color-bg)] text-[var(--sl-color-white)]',
              ]}
            >
              <span class={['font-semibold', state === 'moved' && 'line-through']}>{nm.name}</span>
              <span class="text-[var(--sl-color-gray-3)]"> = </span>
              {#if nm.points != null}
                <span class={['text-[var(--sl-color-gray-2)]', state === 'moved' && 'line-through']}>&rarr; {nm.points}</span>
              {:else}
                <span class={[state === 'moved' && 'line-through']}>{nm.value ?? '??'}</span>
              {/if}
              {#if state === 'moved'}
                <span class="ml-1 no-underline text-xs text-[var(--sl-color-gray-3)]">(moved)</span>
              {:else if state === 'borrow'}
                <span class="ml-1 text-xs text-[var(--sl-color-accent-high)]">(borrow)</span>
              {/if}
            </div>
          {/each}
          {#if currentNames.length === 0}
            <div class="text-sm italic text-[var(--sl-color-gray-3)]">No names in scope.</div>
          {/if}
        </div>
      </div>

      <!-- Column 3: heap objects -->
      <div class="min-w-0">
        <div class="mb-1 text-xs font-semibold uppercase tracking-wide text-[var(--sl-color-gray-3)]">
          Objects
        </div>
        <div class="flex flex-col gap-2">
          {#each currentObjects as obj (obj.id)}
            <div class="rounded-md border border-[var(--sl-color-gray-5)] bg-[var(--sl-color-bg)] px-3 py-2 font-mono text-sm text-[var(--sl-color-white)]">
              <span class="text-[var(--sl-color-gray-3)]">{obj.id}:</span> {obj.value}
            </div>
          {/each}
          {#if currentObjects.length === 0}
            <div class="text-sm italic text-[var(--sl-color-gray-3)]">No heap objects.</div>
          {/if}
        </div>
      </div>
    </div>

    <!-- Controls -->
    <div class="mt-4 flex flex-wrap items-center gap-3">
      <button
        type="button"
        onclick={goPrev}
        disabled={safeIndex <= 0}
        aria-label="Previous step"
        class="rounded-md m-0 border border-[var(--sl-color-gray-5)] bg-[var(--sl-color-bg)] px-3 py-1.5 text-sm text-[var(--sl-color-white)] disabled:opacity-50"
      >
        Previous
      </button>
      <button
        type="button"
        onclick={goNext}
        disabled={safeIndex >= stepCount - 1}
        aria-label="Next step"
        class="rounded-md m-0 border border-[var(--sl-color-gray-5)] bg-[var(--sl-color-bg)] px-3 py-1.5 text-sm text-[var(--sl-color-white)] disabled:opacity-50"
      >
        Next
      </button>
      <span class="text-sm font-mono text-[var(--sl-color-gray-2)]" aria-live="polite">
        Step {stepCount === 0 ? 0 : safeIndex + 1} of {stepCount}
      </span>
    </div>

    {#if currentStep?.caption}
      <p class="mt-3 rounded-md border border-[var(--sl-color-gray-5)] bg-[var(--sl-color-bg)] px-3 py-2 text-sm text-[var(--sl-color-gray-2)]" aria-live="polite">
        {currentStep.caption}
      </p>
    {/if}
  </div>
</div>
