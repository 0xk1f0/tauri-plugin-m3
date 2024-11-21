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
 * Main `tauri-plugin-m3` Utility Class
 */
export class M3 {
    private async colors(
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

    private async apply(colorScheme: Promise<ColorScheme>): Promise<boolean> {
        let scheme = await this.colors(colorScheme);
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
     * Fetch Material3 colors from Android device
     */
    public fetch() {
        let scheme = invoke<ColorScheme>("plugin:m3|colors");
        return {
            /**
             * Return the fetched ColorScheme
             * @example
             * ```javascript
             * import { M3 } from "tauri-plugin-m3";
             *
             * const Material3 = new M3();
             * let colorScheme = await Material3.fetch().colors();
             * ```
             * @return A ColorScheme object or false if unsuccessful
             */
            colors: () => {
                return this.colors(scheme);
            },
            /**
             * Apply all colors in ColorScheme as CSS environment variables
             * @example
             * ```javascript
             * import { M3 } from "tauri-plugin-m3";
             *
             * const Material3 = new M3();
             * await Material3.fetch().apply();
             * ```
             * @return A boolean indicating if successful or not
             */
            apply: () => {
                return this.apply(scheme);
            },
        };
    }
}

/**
 * @deprecated Fetch Material3 colors from Android device
 */
export async function colors(): Promise<ColorScheme | false> {
    return await invoke<ColorScheme>("plugin:m3|colors").then(
        (r: ColorScheme | null) => (r ? ("error" in r ? false : r) : false),
    );
}

/**
 * @deprecated Set all colors in ColorScheme as CSS environment variables
 */
export function updateEnvironment(colorScheme: ColorScheme) {
    for (const [varName, colorValue] of Object.entries(colorScheme)) {
        document.documentElement.style.setProperty(`--${varName}`, colorValue);
    }
}
