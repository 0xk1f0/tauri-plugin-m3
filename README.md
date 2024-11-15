<div align="center">
  <h1>tauri-plugin-m3 </h1>
  <video src="https://github.com/user-attachments/assets/7de52892-75c7-4918-8edd-1d86aadc063b" />
  <p>Android <a href="[https://github.com/tauri-apps/tauri](https://developer.android.com/develop/ui/compose/designsystems/material3)">Material3</a> Plugin for Tauri Apps</p>
</div>

## Features

- Access MaterialYou Dynamic Color Palette
- Automatically enables Android's [EdgeToEdge](https://developer.android.com/develop/ui/views/layout/edge-to-edge) StatusBar and NavigationBar Styling

## Install

Install the Core plugin by adding the following to your `Cargo.toml` file:

`src-tauri/Cargo.toml`

```toml
[dependencies]
tauri-plugin-m3 = { git = "https://github.com/0xk1f0/tauri-plugin-m3" }
```

You can install the JavaScript Guest bindings like this:

```sh
npm install https://github.com/0xk1f0/tauri-plugin-m3
```

## Usage

First you need to register the core plugin with Tauri:

`src-tauri/src/main.rs`

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_m3::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Afterwards all the plugin's APIs are available through the JavaScript guest bindings:

```typescript
import { colors } from "tauri-plugin-m3";
import type { ColorScheme } from "tauri-plugin-m3";

let colorScheme: ColorScheme = await colors();

console.log(colorScheme.primary); // Outputs color in RGBA format f.E. "#F4F678FF"
```

The following colors are available

```typescript
type ColorScheme = {
    primary?: string;
    onPrimary?: string;
    primaryContainer?: string;
    onPrimaryContainer?: string;
    inversePrimary?: string;
    secondary?: string;
    onSecondary?: string;
    secondaryContainer?: string;
    onSecondaryContainer?: string;
    tertiary?: string;
    onTertiary?: string;
    tertiaryContainer?: string;
    onTertiaryContainer?: string;
    background?: string;
    onBackground?: string;
    surface?: string;
    onSurface?: string;
    surfaceVariant?: string;
    onSurfaceVariant?: string;
    inverseSurface?: string;
    inverseOnSurface?: string;
    outline?: string;
};
```

## Global Theming

We can implement automatic global theming of our app via defining a fallback theme in our primary CSS file first

`styles.css`

```css
:root {
    --primary: #bb86fc;
    --onPrimary: #000000;
    --primaryContainer: #3700b3;
    --onPrimaryContainer: #ffffff;
    --inversePrimary: #6200ee;
    --secondary: #03dac6;
    --onSecondary: #000000;
    --secondaryContainer: #00574b;
    --onSecondaryContainer: #ffffff;
    --tertiary: #03a9f4;
    --onTertiary: #000000;
    --tertiaryContainer: #014d73;
    --onTertiaryContainer: #ffffff;
    --background: #121212;
    --onBackground: #e0e0e0;
    --surface: #1e1e1e;
    --onSurface: #e0e0e0;
    --surfaceVariant: #2c2c2c;
    --onSurfaceVariant: #e0e0e0;
    --inverseSurface: #ffffff;
    --inverseOnSurface: #000000;
    --outline: #a0a0a0;
}
```

Then initialize our colors when our app loads for the first time, f.E. in Svelte's `onMount()`

`App.svelte`

```html
<script>
    import { onMount } from 'svelte';
    import { colors, updateEnvironment } from 'tauri-plugin-m3';

    onMount(() => {
        // fetch the colors from device
        colors().then((r) => {
            // overwrite the CSS variables with new colors
            if (r) updateEnvironment(r);
        })
    });
</script>

<div>
    <h1 style="color: var(--primary);">Hello World</h1>
</div>

<style>
    body {
        background-color: var(--background);
    }
</style>
```

This will ensure that our MaterialYou colors get loaded into our predefined CSS variables, but also provides a fallback theme.

Of course this is only one way to do it, feel free to use this plugin like you want! :)

## Credits and Thanks

- [plugins-workspace](https://github.com/tauri-apps/plugins-workspace) - For showing me how to build Tauri Plugins
