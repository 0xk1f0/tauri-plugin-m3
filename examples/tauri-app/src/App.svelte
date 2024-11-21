<script lang="ts">
    import { M3 } from "tauri-plugin-m3";
    import type { ColorScheme } from "tauri-plugin-m3";
    import { onMount } from "svelte";

    let colorScheme: ColorScheme | false = false;

    onMount(async () => {
        const Material3 = new M3();
        // get color values
        colorScheme = await Material3.fetch().colors();
        // update environment
        await Material3.fetch().apply();
    });
</script>

<main class="container">
    <h1 style="margin-top: 4rem;">Tauri M3-Plugin Demo</h1>
    <div>
        {#if colorScheme !== false}
            {#each Object.entries(colorScheme) as [name, value]}
                <h1
                    style="color: #FFFFFF !important; background-color: {value} !important;"
                >
                    {name}
                </h1>
            {/each}
        {:else}
            <h1>MaterialYou unsupported on this device</h1>
        {/if}
    </div>
</main>
