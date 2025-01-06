<div align="center">
    <h1>tauri-plugin-m3</h1>
    <h3>Android <a href="https://developer.android.com/develop/ui/compose/designsystems/material3">Material3/MaterialYou</a> Plugin</h3>
    <a href="https://github.com/0xk1f0/tauri-plugin-m3/releases/latest"><img alt="GitHub Release" src="https://img.shields.io/github/v/release/0xk1f0/tauri-plugin-m3?color=blue&style=flat-square" /></a>
    <a href="https://crates.io/crates/tauri-plugin-m3"><img alt="Crates.io Release" src="https://img.shields.io/crates/v/tauri-plugin-m3?color=orange&style=flat-square" /></a>
    <a href="https://www.npmjs.com/package/tauri-plugin-m3"><img alt="NPM Release" src="https://img.shields.io/npm/v/tauri-plugin-m3?color=green&style=flat-square" /></a>
    <details>
        <summary>Expand Demo</summary>
        <video src="https://github.com/user-attachments/assets/7de52892-75c7-4918-8edd-1d86aadc063b" />
    </details>
</div>

## Features

- Access MaterialYou Dynamic Color Palette
- Automatically enables Android's [EdgeToEdge](https://developer.android.com/develop/ui/views/layout/edge-to-edge) StatusBar and NavigationBar Styling

## Install

Install the Core plugin by adding the following to your `Cargo.toml` file:

`src-tauri/Cargo.toml`

```toml
# via crates.io
[dependencies]
tauri-plugin-m3 = "0.1.1"
# or directly
[dependencies]
tauri-plugin-m3 = { git = "https://github.com/0xk1f0/tauri-plugin-m3" }
```

And then install the JavaScript Guest bindings like this:

```sh
# via npmjs.org
npm install tauri-plugin-m3
# or directly
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
import { M3 } from "tauri-plugin-m3";
import type { ColorScheme } from "tauri-plugin-m3";

let colorScheme = await M3.fetch().colors();

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
    import { M3 } from 'tauri-plugin-m3';

    const Material3 = new M3();

    onMount(async () => {
        await Material3.fetch().apply();
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
