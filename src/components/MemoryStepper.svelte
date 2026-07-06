<!--
  Dual-language memory stepper.
  Walks a reader through a pre-authored sequence of steps that visualize how
  names point at values: Python aliasing (two names, one shared object) versus
  Rust ownership, moves, and borrows. This component does NOT parse or execute
  code; it renders an authored trace passed in as a prop, which is robust and
  reliable. Pure client-side, no network, no external libraries.

  Layout: a single vertical flow, code (full width) -> step explanation ->
  controls (previous / next / scrubber) -> a "names in scope" table -> the heap
  objects those names point at. Names sharing an object show the same reference,
  which is how aliasing is made visible.
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

  function onScrub(event) {
    stepIndex = Number(event.currentTarget.value);
  }

  function onKeydown(event) {
    if (event.key === 'ArrowLeft') {
      goPrev();
      event.preventDefault();
    } else if (event.key === 'ArrowRight') {
      goNext();
      event.preventDefault();
    }
  }

  function nameState(nm) {
    return nm && (nm.state === 'moved' || nm.state === 'borrow') ? nm.state : 'owned';
  }

  // The short tag shown in the Status column. A name that refers to an object is
  // an "owner" (or moved / borrow); a name that holds a scalar directly is a "value".
  function statusLabel(nm) {
    const state = nameState(nm);
    if (state === 'moved') return 'moved';
    if (state === 'borrow') return 'borrow';
    return nm && nm.points != null ? 'owner' : 'value';
  }

  // Render an object id like "obj1" as the friendlier "object 1", while leaving
  // any other id untouched. Used identically in the table and the heap strip so
  // a name's reference always matches the object it points at.
  function prettyId(id) {
    const match = String(id).match(/^obj(\d+)$/);
    return match ? `object ${match[1]}` : String(id);
  }
</script>

<div>
  <div class="hidden print:block text-sm italic text-[var(--sl-color-gray-4)]">
    Interactive memory stepper example is not available when printing. Please view this page online.
  </div>

  <div
    class="print:hidden rounded-lg border border-[var(--sl-color-hairline)] bg-[var(--sl-color-bg-nav)] p-4 focus-visible:outline focus-visible:outline-2 focus-visible:outline-[var(--sl-color-accent)]"
    role="group"
    aria-label={title ? `Memory stepper: ${title}` : 'Memory stepper'}
    tabindex="0"
    onkeydown={onKeydown}
  >
    {#if title}
      <h3 class="m-0 mb-3 text-base font-semibold text-[var(--sl-color-white)]">{title}</h3>
    {/if}

    <!-- Code, full width, current line highlighted -->
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

    <!-- What just happened -->
    <div class="mt-3 rounded-md border border-[var(--sl-color-gray-5)] bg-[var(--sl-color-bg)] px-3 py-2">
      <div class="font-mono text-xs text-[var(--sl-color-gray-3)]" aria-live="polite">
        Step {stepCount === 0 ? 0 : safeIndex + 1} of {stepCount}
      </div>
      {#if currentStep?.caption}
        <p class="mt-1 mb-0 text-sm text-[var(--sl-color-gray-1)]" aria-live="polite">{currentStep.caption}</p>
      {/if}
    </div>

    <!-- Controls: previous / next / scrubber -->
    <div class="mt-3 flex flex-wrap items-center gap-3">
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
      <input
        type="range"
        min="0"
        max={Math.max(0, stepCount - 1)}
        value={safeIndex}
        oninput={onScrub}
        disabled={stepCount <= 1}
        aria-label="Step through the trace"
        class="mt-0 min-w-[8rem] flex-1"
        style="accent-color: var(--sl-color-accent);"
      />
    </div>

    <!-- Names currently in scope -->
    <div class="mt-4 mb-1 text-xs font-semibold uppercase tracking-wide text-[var(--sl-color-gray-3)]">
      In scope
    </div>
    <div class="overflow-x-auto rounded-md border border-[var(--sl-color-gray-5)]">
      <table class="w-full border-collapse text-sm">
        <thead>
          <tr class="text-left">
            <th class="px-3 py-2 font-mono text-xs font-medium uppercase tracking-wide text-[var(--sl-color-gray-3)]">Name</th>
            <th class="px-3 py-2 font-mono text-xs font-medium uppercase tracking-wide text-[var(--sl-color-gray-3)]">Holds</th>
            <th class="px-3 py-2 font-mono text-xs font-medium uppercase tracking-wide text-[var(--sl-color-gray-3)]">Status</th>
          </tr>
        </thead>
        <tbody>
          {#each currentNames as nm (nm.name)}
            {@const state = nameState(nm)}
            <tr class="border-t border-[var(--sl-color-gray-5)]">
              <td class={['px-3 py-2 font-mono font-semibold', state === 'moved' ? 'text-[var(--sl-color-gray-4)] line-through' : 'text-[var(--sl-color-white)]']}>
                {nm.name}
              </td>
              <td class="px-3 py-2 font-mono">
                {#if nm.points != null}
                  <span class={state === 'moved' ? 'text-[var(--sl-color-gray-4)] line-through' : 'text-[var(--sl-color-accent-high)]'}>
                    <span aria-hidden="true">&rarr;</span> {prettyId(nm.points)}
                  </span>
                {:else}
                  <span class={state === 'moved' ? 'text-[var(--sl-color-gray-4)] line-through' : 'text-[var(--sl-color-gray-2)]'}>{nm.value ?? '??'}</span>
                {/if}
              </td>
              <td class="px-3 py-2">
                <span
                  class={[
                    'inline-block rounded-full px-2 py-0.5 font-mono text-xs',
                    state === 'borrow' && 'bg-[var(--sl-color-accent-low)] text-[var(--sl-color-accent-high)]',
                    state === 'moved' && 'bg-[var(--sl-color-gray-6)] text-[var(--sl-color-gray-4)]',
                    state === 'owned' && 'bg-[var(--sl-color-gray-6)] text-[var(--sl-color-gray-3)]',
                  ]}
                >
                  {statusLabel(nm)}
                </span>
              </td>
            </tr>
          {/each}
          {#if currentNames.length === 0}
            <tr>
              <td class="px-3 py-2 text-sm italic text-[var(--sl-color-gray-3)]" colspan="3">No names in scope.</td>
            </tr>
          {/if}
        </tbody>
      </table>
    </div>

    <!-- Heap objects the names point at (only shown when there are any) -->
    {#if currentObjects.length > 0}
      <div class="mt-4 mb-1 text-xs font-semibold uppercase tracking-wide text-[var(--sl-color-gray-3)]">
        In memory
      </div>
      <div class="flex flex-wrap gap-2">
        {#each currentObjects as obj (obj.id)}
          <div class="rounded-md border border-dashed border-[var(--sl-color-accent)] bg-[var(--sl-color-accent-low)] px-3 py-1.5 font-mono text-sm text-[var(--sl-color-white)]">
            <span class="text-[var(--sl-color-accent-high)]">{prettyId(obj.id)}</span>&nbsp;&nbsp;{obj.value}
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>
