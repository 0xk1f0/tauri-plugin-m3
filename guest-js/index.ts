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

export type InsetsScheme = {
    rawInsetTop?: number;
    rawInsetBottom?: number;
    rawInsetLeft?: number;
    rawInsetRight?: number;
    adjustedInsetTop?: number;
    adjustedInsetBottom?: number;
    adjustedInsetLeft?: number;
    adjustedInsetRight?: number;
    scaleFactor?: number;
};

export type BarColorScheme = {
    color?: string;
};

async function getColors(
    colorScheme: Promise<ColorScheme>,
): Promise<ColorScheme | false> {
    try {
        let scheme = await colorScheme;
        if ("error" in scheme) return false;
        return scheme;
    } catch {
        return false;
    }
}

async function applyScheme(colorScheme: Promise<ColorScheme>): Promise<boolean> {
    let scheme = await getColors(colorScheme);
    if (!scheme) return false;
    for (const [varName, colorValue] of Object.entries(colorScheme)) {
        document.documentElement.style.setProperty(
            `--${varName}`,
            colorValue,
        );
    }
    return true;
}

/**
 * Main `tauri-plugin-m3` Utility Class
 */
export class M3 {
    /**
     * Get all inset properties for proper Edge-To-Edge Display
     * @example
     * ```javascript
     * import { M3 } from "tauri-plugin-m3";
     *
     * let insets = await M3.getInsets();
     * ```
     * @return A InsetsScheme object or false if unsuccessful
     */
    public static async getInsets(): Promise<InsetsScheme | false> {
        try {
            return await invoke<InsetsScheme>("plugin:m3|insets");
        } catch {
            return false;
        }
    }

    /**
     * Set the current status and navigation bar color
     * @example
     * ```javascript
     * import { M3 } from "tauri-plugin-m3";
     * 
     * // either "dark", "light" or "system"
     * // default is "system"
     * let status = await M3.barColor("dark");
     * ```
     * @return A BarColorScheme object or false if unsuccessful
     */
    public static async barColor(color: "dark" | "light" | "system" = "system"): Promise<BarColorScheme | false> {
        try {
            return await invoke<BarColorScheme>("plugin:m3|bar_color", { color });
        } catch {
            return false;
        }
    }

    /**
     * Fetch Material3 colors from Android device
     */
    public static fetch(theme: "dark" | "light" | "system" = "system") {
        let scheme = invoke<ColorScheme>("plugin:m3|colors", { theme });
        return {
            /**
             * Return the fetched ColorScheme
             * @example
             * ```javascript
             * import { M3 } from "tauri-plugin-m3";
             * 
             * // either "dark", "light" or "system"
             * // default is "system"
             * let colorScheme = await M3.fetch("dark").colors();
             * ```
             * @return A ColorScheme object or false if unsuccessful
             */
            colors: () => {
                return getColors(scheme);
            },
            /**
             * Apply all colors in ColorScheme as CSS environment variables
             * @example
             * ```javascript
             * import { M3 } from "tauri-plugin-m3";
             *
             * // either "dark", "light" or "system"
             * // default is "system"
             * await M3.fetch("dark").apply();
             * ```
             * @return A boolean indicating if successful or not
             */
            apply: () => {
                return applyScheme(scheme);
            },
        };
    }
}
