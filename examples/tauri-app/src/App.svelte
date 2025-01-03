<script lang="ts">
    import { M3, getOffsets } from "tauri-plugin-m3";
    import type { ColorScheme, OffsetsScheme } from "tauri-plugin-m3";
    import { onMount } from "svelte";

    let colorScheme: ColorScheme | false = false;
    let offsets: OffsetsScheme | false = false;

    onMount(async () => {
        const Material3 = new M3();
        // get color values
        colorScheme = await Material3.fetch().colors();
        // get offsets
        offsets = await getOffsets();
        // update environment
        await Material3.fetch().apply();
    });
</script>

<main class="container">
    <h1>Tauri M3-Plugin Demo</h1>
    <div>
        {#if offsets !== false}
            <h1>Offsets</h1>
            {#each Object.entries(offsets) as [name, value]}
                <h2
                    style="color: #FFFFFF !important;"
                >
                    {name}: {value}
                </h2>
            {/each}
        {:else}
            <h1>Edge-To-Edge offsets unsupported on this device</h1>
        {/if}
    </div>
    <div>
        {#if colorScheme !== false}
            <h1>Colors</h1>
            {#each Object.entries(colorScheme) as [name, value]}
                <h2
                    style="color: #FFFFFF !important; background-color: {value} !important;"
                >
                    {name}
                </h2>
            {/each}
        {:else}
            <h1>MaterialYou unsupported on this device</h1>
        {/if}
    </div>
</main>
