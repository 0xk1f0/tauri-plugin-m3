<script>
    import { colors } from "tauri-plugin-m3";
    import { onMount } from "svelte";

    let colorScheme = null;

    onMount(() => {
        colors()
            .then((value) => {
                if (value) colorScheme = value;
            })
            .catch();
    });
</script>

<main class="container">
    <h1 style="margin-top: 4rem;">Tauri M3-Plugin Demo</h1>
    <div>
        {#if colorScheme !== null}
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
