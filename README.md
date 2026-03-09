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
- Change Status Bar and Navigation Bar Color
- Access Window Insets to align EdgeToEdge layout correctly

## Install

Install the Core plugin by adding the following to your `Cargo.toml` file:

`src-tauri/Cargo.toml`

```toml
# via crates.io
[dependencies]
tauri-plugin-m3 = "0.2.2"
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

Then set the default permissions for the plugin:

`src-tauri/capabilities/default.json`

```json
{
    "permissions": [
        "m3:default"
    ]
}
```

Afterwards all the plugin's APIs are available through the JavaScript guest bindings:

```typescript
import { M3 } from "tauri-plugin-m3";

// get the full material color palette
// choose either "dark", "light" or "system" (default)
let colorScheme = await M3.getColors("system");
console.log(colorScheme.primary); // "#F4F678FF"

// apply colors to CSS variables
// choose either "dark", "light" or "system" (default)
let isSuccess = await M3.applyColors("light");
console.log(isSuccess); // "true"

// get insets for compensating EdgeToEdge display
// either already scale compensated or raw
let deviceInsets = await M3.getInsets();
console.log(deviceInsets.adjustedInsetTop); // f.E. 96

// set the status and navigation bar color
// choose either "dark", "light" or "system" (default)
let isSuccess = await M3.setBarColor("dark");
console.log(isSuccess); // "true"
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
    surfaceTint?: string;
    inverseSurface?: string;
    inverseOnSurface?: string;
    error?: string;
    onError?: string;
    errorContainer?: string;
    onErrorContainer?: string;
    outline?: string;
    outlineVariant?: string;
    scrim?: string;
    surfaceBright?: string;
    surfaceDim?: string;
    surfaceContainer?: string;
    surfaceContainerHigh?: string;
    surfaceContainerHighest?: string;
    surfaceContainerLow?: string;
    surfaceContainerLowest?: string;
    primaryFixed?: string;
    primaryFixedDim?: string;
    onPrimaryFixed?: string;
    onPrimaryFixedVariant?: string;
    secondaryFixed?: string;
    secondaryFixedDim?: string;
    onSecondaryFixed?: string;
    onSecondaryFixedVariant?: string;
    tertiaryFixed?: string;
    tertiaryFixedDim?: string;
    onTertiaryFixed?: string;
    onTertiaryFixedVariant?: string;
};
```

## Transparent Navigation Bar

To get a fully transaprent navigation bar, add the following in your app

`src-tauri/gen/android/app/src/main/res/values/themes.xml`

```diff
<resources xmlns:tools="http://schemas.android.com/tools">
    <!-- Base application theme. -->
    <style name="Theme.stority" parent="Theme.MaterialComponents.DayNight.NoActionBar">
+        <item name="android:enforceNavigationBarContrast">false</item>
+        <item name="android:enforceStatusBarContrast">false</item>
    </style>
</resources>
```

## Global Theming

We can implement automatic global theming of our app via defining a fallback theme in our primary CSS file first

`styles.css`

```css
:root {
    --primary: #AAC7FFFF;
	--onPrimary: #0D2F5FFF;
	--primaryContainer: #294677FF;
	--onPrimaryContainer: #D6E3FFFF;
	--inversePrimary: #425E91FF;
	--secondary: #BEC6DCFF;
	--onSecondary: #283041FF;
	--secondaryContainer: #3E4759FF;
	--onSecondaryContainer: #DAE2F9FF;
	--tertiary: #DDBCDFFF;
	--onTertiary: #3F2844FF;
	--tertiaryContainer: #573E5BFF;
	--onTertiaryContainer: #FAD7FCFF;
	--background: #11131AFF;
	--onBackground: #E0E2ECFF;
	--surface: #11131AFF;
	--onSurface: #E0E2ECFF;
	--surfaceVariant: #44464EFF;
	--onSurfaceVariant: #C5C6D0FF;
	--surfaceTint: #AAC7FFFF;
	--inverseSurface: #E0E2ECFF;
	--inverseOnSurface: #2D3038FF;
	--error: #F2B8B5FF;
	--onError: #601410FF;
	--errorContainer: #8C1D18FF;
	--onErrorContainer: #F9DEDCFF;
	--outline: #8E9099FF;
	--outlineVariant: #44464EFF;
	--scrim: #000000FF;
	--surfaceBright: #373940FF;
	--surfaceDim: #11131AFF;
	--surfaceContainer: #1D1F26FF;
	--surfaceContainerHigh: #272A31FF;
	--surfaceContainerHighest: #32353CFF;
	--surfaceContainerLow: #1B1B1DFF;
	--surfaceContainerLowest: #0C0E14FF;
	--primaryFixed: #D6E3FFFF;
	--primaryFixedDim: #AAC7FFFF;
	--onPrimaryFixed: #001A41FF;
	--onPrimaryFixedVariant: #294677FF;
	--secondaryFixed: #DAE2F9FF;
	--secondaryFixedDim: #BEC6DCFF;
	--onSecondaryFixed: #131C2BFF;
	--onSecondaryFixedVariant: #3E4759FF;
	--tertiaryFixed: #FAD7FCFF;
	--tertiaryFixedDim: #DDBCDFFF;
	--onTertiaryFixed: #29132EFF;
	--onTertiaryFixedVariant: #573E5BFF;
}
```

Then initialize our colors when our app loads for the first time, f.E. in Svelte's `onMount()`

`App.svelte`

```html
<script>
    import { onMount } from 'svelte';
    import { M3 } from 'tauri-plugin-m3';

    onMount(async () => {
        await M3.applyColors("system");
        await M3.setBarColor("system");
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
