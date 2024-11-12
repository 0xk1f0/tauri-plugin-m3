# tauri-plugin-m3 

Android [Material3](https://developer.android.com/develop/ui/compose/designsystems/material3)  Plugin for Tauri Apps

## Features

- Access MaterialYou Dynamic Color Palette
- Automatically enables Android's [EdgeToEdge](https://developer.android.com/develop/ui/views/layout/edge-to-edge) StatusBar and NavigationBar Styling

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
import { colors } from "tauri-plugin-m3-api";
import type { ColorScheme } from "tauri-plugin-m3-api";

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

## Credits and Thanks

- [plugins-workspace](https://github.com/tauri-apps/plugins-workspace) - For showing me how to build Tauri Plugins
