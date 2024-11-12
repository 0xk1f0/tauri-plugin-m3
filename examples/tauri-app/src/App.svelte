<script>
    import { colors } from "tauri-plugin-m3-api";
    import { onMount } from "svelte";

    let error = null;
    let colorScheme = null;

    onMount(() => {
        colors()
            .then((value) => {
                colorScheme = value;
            })
            .catch((value) => (error = value));
    });
</script>

<main class="container">
    <h1>Tauri M3-Plugin Demo</h1>
    <div>
        {#if colorScheme !== null}
            {#each Object.entries(colorScheme) as [name, value]}
                <h1
                    style="color: #FFFFFF !important; background-color: {value} !important;"
                >
                    {name}
                </h1>
            {/each}
        {/if}
        {#if error !== null}
            <h1>{JSON.stringify(error)}</h1>
        {/if}
    </div>
</main>
