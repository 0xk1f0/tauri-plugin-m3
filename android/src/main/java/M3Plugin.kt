package com.plugin.m3

import android.app.Activity
import android.webkit.WebView
import android.view.Window
import androidx.core.view.ViewCompat
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat
import android.os.Build
import android.graphics.Color
import android.content.res.Configuration
import android.content.Context
import androidx.compose.ui.graphics.toArgb
import androidx.compose.material3.dynamicLightColorScheme
import androidx.compose.material3.dynamicDarkColorScheme
import androidx.compose.material3.ColorScheme
import kotlin.math.round
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@InvokeArg
class ColorsOptions {
    lateinit var theme: String
}

@InvokeArg
class BarOptions {
    lateinit var color: String
}

@TauriPlugin
class M3Plugin(private val activity: Activity): Plugin(activity) {
    @Command
    fun colors(invoke: Invoke) {
        val ret = JSObject()
        val args = invoke.parseArgs(ColorsOptions::class.java)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            val context = activity.getApplication().getApplicationContext()
            val colorScheme: ColorScheme
            if (args.theme == "dark" || args.theme == "light") {
                colorScheme = if (args.theme == "dark") dynamicDarkColorScheme(context) else dynamicLightColorScheme(context)
            } else {
                colorScheme = if (isNightMode(context)) dynamicDarkColorScheme(context) else dynamicLightColorScheme(context)
            }
            ret.put("primary", argbToRgba(colorScheme.primary.toArgb()))
            ret.put("onPrimary", argbToRgba(colorScheme.onPrimary.toArgb()))
            ret.put("primaryContainer", argbToRgba(colorScheme.primaryContainer.toArgb()))
            ret.put("onPrimaryContainer", argbToRgba(colorScheme.onPrimaryContainer.toArgb()))
            ret.put("inversePrimary", argbToRgba(colorScheme.inversePrimary.toArgb()))
            ret.put("secondary", argbToRgba(colorScheme.secondary.toArgb()))
            ret.put("onSecondary", argbToRgba(colorScheme.onSecondary.toArgb()))
            ret.put("secondaryContainer", argbToRgba(colorScheme.secondaryContainer.toArgb()))
            ret.put("onSecondaryContainer", argbToRgba(colorScheme.onSecondaryContainer.toArgb()))
            ret.put("tertiary", argbToRgba(colorScheme.tertiary.toArgb()))
            ret.put("onTertiary", argbToRgba(colorScheme.onTertiary.toArgb()))
            ret.put("tertiaryContainer", argbToRgba(colorScheme.tertiaryContainer.toArgb()))
            ret.put("onTertiaryContainer", argbToRgba(colorScheme.onTertiaryContainer.toArgb()))
            ret.put("background", argbToRgba(colorScheme.background.toArgb()))
            ret.put("onBackground", argbToRgba(colorScheme.onBackground.toArgb()))
            ret.put("surface", argbToRgba(colorScheme.surface.toArgb()))
            ret.put("onSurface", argbToRgba(colorScheme.onSurface.toArgb()))
            ret.put("surfaceVariant", argbToRgba(colorScheme.surfaceVariant.toArgb()))
            ret.put("onSurfaceVariant", argbToRgba(colorScheme.onSurfaceVariant.toArgb()))
            ret.put("surfaceTint", argbToRgba(colorScheme.surfaceTint.toArgb()))
            ret.put("inverseSurface", argbToRgba(colorScheme.inverseSurface.toArgb()))
            ret.put("inverseOnSurface", argbToRgba(colorScheme.inverseOnSurface.toArgb()))
            ret.put("error", argbToRgba(colorScheme.error.toArgb()))
            ret.put("onError", argbToRgba(colorScheme.onError.toArgb()))
            ret.put("errorContainer", argbToRgba(colorScheme.errorContainer.toArgb()))
            ret.put("onErrorContainer", argbToRgba(colorScheme.onErrorContainer.toArgb()))
            ret.put("outline", argbToRgba(colorScheme.outline.toArgb()))
            ret.put("outlineVariant", argbToRgba(colorScheme.outlineVariant.toArgb()))
            ret.put("scrim", argbToRgba(colorScheme.scrim.toArgb()))
            ret.put("surfaceBright", argbToRgba(colorScheme.surfaceBright.toArgb()))
            ret.put("surfaceDim", argbToRgba(colorScheme.surfaceDim.toArgb()))
            ret.put("surfaceContainer", argbToRgba(colorScheme.surfaceContainer.toArgb()))
            ret.put("surfaceContainerHigh", argbToRgba(colorScheme.surfaceContainerHigh.toArgb()))
            ret.put("surfaceContainerHighest", argbToRgba(colorScheme.surfaceContainerHighest.toArgb()))
            ret.put("surfaceContainerLow", argbToRgba(colorScheme.surfaceContainerLow.toArgb()))
            ret.put("surfaceContainerLowest", argbToRgba(colorScheme.surfaceContainerLowest.toArgb()))
            ret.put("primaryFixed", argbToRgba(colorScheme.primaryFixed.toArgb()))
            ret.put("primaryFixedDim", argbToRgba(colorScheme.primaryFixedDim.toArgb()))
            ret.put("onPrimaryFixed", argbToRgba(colorScheme.onPrimaryFixed.toArgb()))
            ret.put("onPrimaryFixedVariant", argbToRgba(colorScheme.onPrimaryFixedVariant.toArgb()))
            ret.put("secondaryFixed", argbToRgba(colorScheme.secondaryFixed.toArgb()))
            ret.put("secondaryFixedDim", argbToRgba(colorScheme.secondaryFixedDim.toArgb()))
            ret.put("onSecondaryFixed", argbToRgba(colorScheme.onSecondaryFixed.toArgb()))
            ret.put("onSecondaryFixedVariant", argbToRgba(colorScheme.onSecondaryFixedVariant.toArgb()))
            ret.put("tertiaryFixed", argbToRgba(colorScheme.tertiaryFixed.toArgb()))
            ret.put("tertiaryFixedDim", argbToRgba(colorScheme.tertiaryFixedDim.toArgb()))
            ret.put("onTertiaryFixed", argbToRgba(colorScheme.onTertiaryFixed.toArgb()))
            ret.put("onTertiaryFixedVariant", argbToRgba(colorScheme.onTertiaryFixedVariant.toArgb()))
        } else {
            invoke.reject("MaterialYou not supported on this device!")
            return
        }
        invoke.resolve(ret)
    }
    @Command
    fun insets(invoke: Invoke) {
        val ret = JSObject()
        val window = activity.getWindow()
        val context = activity.getApplication().getApplicationContext()
        val displayMetrics = context.getResources().getDisplayMetrics()
        val rootInsets = ViewCompat.getRootWindowInsets(window.decorView)!!
        val insets = rootInsets.getInsets(WindowInsetsCompat.Type.systemBars())
        ret.put("adjustedInsetTop", round(insets.top / displayMetrics.density).toInt())
        ret.put("adjustedInsetBottom", round(insets.bottom / displayMetrics.density).toInt())
        ret.put("adjustedInsetLeft", round(insets.left / displayMetrics.density).toInt())
        ret.put("adjustedInsetRight", round(insets.right / displayMetrics.density).toInt())
        ret.put("rawInsetTop", insets.top)
        ret.put("rawInsetBottom", insets.bottom)
        ret.put("rawInsetLeft", insets.left)
        ret.put("rawInsetRight", insets.right)
        ret.put("scaleFactor", displayMetrics.density)
        invoke.resolve(ret)
    }
    @Command
    fun barColor(invoke: Invoke) {
        val ret = JSObject()
        val args = invoke.parseArgs(BarOptions::class.java)
        val window = activity.getWindow()
        val context = activity.getApplication().getApplicationContext()
        if (args.color == "dark" || args.color == "light") {
            if (args.color == "dark") setLightStatusBar(window, true) else setLightStatusBar(window, false)
        } else {
            if (isNightMode(context)) setLightStatusBar(window, false) else setLightStatusBar(window, true)
        }
        ret.put("color", args.color)
        invoke.resolve(ret)
    }

    private fun setLightStatusBar(window: Window, isLight: Boolean) {
        val rootView = window.getDecorView().getRootView()
        WindowCompat.getInsetsController(window, rootView).isAppearanceLightStatusBars = isLight
        WindowCompat.getInsetsController(window, rootView).isAppearanceLightNavigationBars = isLight
    }

    private fun argbToRgba(argb: Int): String {
        val hex = Integer.toHexString(argb)
        return String.format("#%s%s", hex.substring(2), hex.substring(0, 2)).uppercase()
    }

    private fun isNightMode(context: Context): Boolean {
        return when (context.getResources().getConfiguration().uiMode and Configuration.UI_MODE_NIGHT_MASK) {
            Configuration.UI_MODE_NIGHT_YES -> true
            Configuration.UI_MODE_NIGHT_NO -> false
            Configuration.UI_MODE_NIGHT_UNDEFINED -> false
            else -> false
        }
    }
}
