import { invoke } from "@tauri-apps/api/core";

export type ColorScheme = {
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

/**
 * Fetch Material3 colors from Android device
 */
export async function colors(): Promise<ColorScheme | false> {
    return await invoke<ColorScheme>("plugin:m3|colors").then(
        (r: ColorScheme | null) => (r ? ("error" in r ? false : r) : false),
    );
}

/**
 * Set all colors in ColorScheme as CSS environment variables
 */
export function updateEnvironment(colorScheme: ColorScheme) {
    for (const [varName, colorValue] of Object.entries(colorScheme)) {
        document.documentElement.style.setProperty(`--${varName}`, colorValue);
    }
}
