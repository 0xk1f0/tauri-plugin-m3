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

export async function colors(): Promise<ColorScheme | null> {
    return await invoke<ColorScheme>("plugin:m3|colors").then((r) =>
        r ? r : null,
    );
}
