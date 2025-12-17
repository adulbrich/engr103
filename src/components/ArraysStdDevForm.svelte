<script>
    // Sample Standard Deviation simulator with adjustable N
    let N = 10;
    let numbers = Array.from({ length: N }, (_, i) => i + 1);
    let error = "";

    // Validate N
    $: {
        if (N < 2) {
            error = "N must be at least 2.";
        } else if (N > 1000) {
            error = "N is too large (max 1000).";
        } else {
            error = "";
        }
    }

    // Resize numbers array when N changes, preserving values where possible
    $: if (numbers.length !== N) {
        if (N > numbers.length) {
            const start = numbers.length;
            numbers = [
                ...numbers,
                ...Array.from(
                    { length: N - numbers.length },
                    (_, i) => start + i + 1,
                ),
            ];
        } else {
            numbers = numbers.slice(0, N);
        }
    }

    // Helpers
    function mean(arr) {
        return arr.length === 0
            ? 0
            : arr.reduce((s, v) => s + Number(v), 0) / arr.length;
    }

    function sampleStdDev(arr) {
        if (arr.length < 2) return 0;
        const m = mean(arr);
        const sumSq = arr.reduce((s, v) => s + (Number(v) - m) ** 2, 0);
        return Math.sqrt(sumSq / (arr.length - 1));
    }

    $: stddev = sampleStdDev(numbers);

    function reset() {
        N = 10;
        numbers = Array.from({ length: N }, (_, i) => i + 1);
        error = "";
    }

    function fillLinear() {
        numbers = numbers.map((n, i) => i + 1);
    }

    function fillRandom(min = 0, max = 100) {
        numbers = numbers.map(
            () => +(Math.random() * (max - min) + min).toFixed(5),
        );
    }
</script>

<div
    class="w-full max-w-lg mx-auto p-4 mt-4 rounded border print:hidden dark:bg-black dark:text-white"
>
    <h3 class="text-xl font-medium mb-2">Standard Deviation 📊</h3>

    <div class="my-4 flex items-end justify-between gap-2">
        <div class="flex flex-col items-start gap-2 m-0">
            <label for="sample-size" class="text-sm">Sample Size</label>
            <input
                id="sample-size"
                type="number"
                min="2"
                max="1000"
                bind:value={N}
                class="w-20 border rounded p-2"
            />
        </div>

        <button
            class="m-0 px-4 py-2 rounded bg-accent-600 text-white"
            on:click={reset}>Restore Defaults</button
        >
    </div>

    <p class="text-sm mb-4">
        Enter values below and the simulator will compute the <em>sample</em> standard
        deviation (N - 1 in denominator).
    </p>

    {#if error}
        <p class="text-red-600 mb-2">{error}</p>
    {/if}

    <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 my-4">
        {#each numbers as value, i}
            <div class="m-0 flex items-center justify-end gap-2">
                <label class="text-xs"
                    >#{i + 1}
                    <input
                        class="text-base border rounded p-2"
                        type="number"
                        step="any"
                        bind:value={numbers[i]}
                    /></label
                >
            </div>
        {/each}
    </div>

    <div class="rounded p-4 bg-gray-50 dark:bg-gray-800 flex flex-col gap-2">
        <div class="flex justify-between">
            <span>Mean</span>
            <span>{mean(numbers).toFixed(5)}</span>
        </div>
        <div class="flex justify-between">
            <span>Standard Deviation</span>
            <span>{stddev.toFixed(5)}</span>
        </div>
    </div>

    <div class="mt-4 flex items-center justify-end gap-2">
        <button class="m-0 px-4 py-2 rounded border" on:click={fillLinear}
            >Fill 1–N</button
        >
        <button
            class="m-0 px-4 py-2 rounded border"
            on:click={() => fillRandom()}>Random Values</button
        >
    </div>
</div>
