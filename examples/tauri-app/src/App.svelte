<script lang="ts">
    import { M3 } from "tauri-plugin-m3";
    import type { ColorScheme, InsetsScheme } from "tauri-plugin-m3";
    import { onMount } from "svelte";

    let colorScheme: ColorScheme | false = false;
    let insets: InsetsScheme | false = false;
    let selectedTheme: string = "system";

    onMount(async () => {
        // get color values
        colorScheme = await M3.fetch().colors();
        // get insets
        insets = await M3.getInsets();
        // update environment
        await M3.fetch().apply();
    });

    async function switchTheme() {
        if (
            selectedTheme == "dark" ||
            selectedTheme == "light" ||
            selectedTheme == "system"
        ) {
            colorScheme = await M3.fetch(selectedTheme).colors();
            await M3.fetch().apply();
        }
    }
</script>

<main class="container">
    <div
        style="margin: 0; padding: 0; margin-top: {insets
            ? insets.adjustedInsetTop
            : 0}px !important;"
    >
        <h1 style="margin: 0; padding: 0;">Tauri M3-Plugin Demo</h1>
    </div>
    <div style="margin: 0; padding: 0;">
        {#if insets !== false}
            <h1>Insets</h1>
            {#each Object.entries(insets) as [name, value]}
                <p style="color: #FFFFFF !important; margin-top: 4px;">
                    {name}: {value}
                </p>
            {/each}
        {:else}
            <h1>Edge-To-Edge insets unsupported on this device</h1>
        {/if}
    </div>
    <div style="margin: 0; padding: 0;">
        <h1>Theme</h1>
        <select bind:value={selectedTheme} id="theme" on:change={switchTheme}>
            <option value="system" selected>System</option>
            <option value="dark">Dark</option>
            <option value="light">Light</option>
        </select>
    </div>
    <div
        style="margin: 0; padding: 0; margin-bottom: {insets
            ? insets.adjustedInsetBottom
            : 0}px !important;"
    >
        {#if colorScheme !== false}
            <h1>Colors</h1>
            {#each Object.entries(colorScheme) as [name, value]}
                <p
                    style="color: #FFFFFF !important; background-color: {value} !important; margin-top: 4px;"
                >
                    {name}
                </p>
            {/each}
        {:else}
            <h1>MaterialYou unsupported on this device</h1>
        {/if}
    </div>
</main>
