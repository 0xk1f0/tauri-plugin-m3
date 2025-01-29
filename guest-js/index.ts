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
     * let isSuccess = await M3.setBarColor("dark");
     * ```
     * @return A boolean indicating if successful or not
     */
    public static async setBarColor(
        color: "dark" | "light" | "system" = "system",
    ): Promise<Boolean> {
        try {
            let result = await invoke<BarColorScheme>("plugin:m3|bar_color", {
                color,
            });
            return "error" in result ? false : true;
        } catch {
            return false;
        }
    }

    /**
     * Return the fetched ColorScheme
     * @example
     * ```javascript
     * import { M3 } from "tauri-plugin-m3";
     *
     * // either "dark", "light" or "system"
     * // default is "system"
     * let colorScheme = await M3.getColors("dark");
     * ```
     * @return A ColorScheme object or false if unsuccessful
     */
    public static async getColors(
        theme: "dark" | "light" | "system" = "system",
    ): Promise<ColorScheme | false> {
        try {
            let scheme: ColorScheme = await invoke<ColorScheme>(
                "plugin:m3|colors",
                { theme },
            );
            if ("error" in scheme) return false;
            return scheme;
        } catch {
            return false;
        }
    }

    /**
     * Apply all colors in ColorScheme as CSS environment variables
     * @example
     * ```javascript
     * import { M3 } from "tauri-plugin-m3";
     *
     * // either "dark", "light" or "system"
     * // default is "system"
     * let isSuccess = await M3.applyColors("dark");
     * ```
     * @return A boolean indicating if successful or not
     */
    public static async applyColors(
        theme: "dark" | "light" | "system" = "system",
    ): Promise<Boolean> {
        try {
            let scheme: ColorScheme = await invoke<ColorScheme>(
                "plugin:m3|colors",
                { theme },
            );
            if (!scheme || "error" in scheme) return false;
            for (const [varName, colorValue] of Object.entries(scheme)) {
                document.documentElement.style.setProperty(
                    `--${varName}`,
                    colorValue,
                );
            }
            return true;
        } catch {
            return false;
        }
    }
}
