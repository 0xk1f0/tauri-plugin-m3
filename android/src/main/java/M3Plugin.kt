package com.plugin.m3

import android.app.Activity
import android.webkit.WebView
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
import kotlin.math.round
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@TauriPlugin
class M3Plugin(private val activity: Activity): Plugin(activity) {
    override fun load(webView: WebView) {
        val window = activity.getWindow()
        val context = activity.getApplication().getApplicationContext()
        val darkTheme = isUsingNightMode(context)
        WindowCompat.setDecorFitsSystemWindows(window, false)
        val controller = WindowCompat.getInsetsController(window, window.decorView)
        if (!darkTheme) {
            controller.setAppearanceLightStatusBars(true)
            controller.setAppearanceLightNavigationBars(true)
        } else {
            controller.setAppearanceLightStatusBars(false)
            controller.setAppearanceLightNavigationBars(false)
        }
        window.apply {
            statusBarColor = Color.TRANSPARENT;
            navigationBarColor = Color.TRANSPARENT;
        }
    }
    @Command
    fun colors(invoke: Invoke) {
        val ret = JSObject()
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            val context = activity.getApplication().getApplicationContext()
            val isDarkTheme = isUsingNightMode(context)
            val colorScheme = if (isDarkTheme) dynamicDarkColorScheme(context) else dynamicLightColorScheme(context)
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
            ret.put("inverseSurface", argbToRgba(colorScheme.inverseSurface.toArgb()))
            ret.put("inverseOnSurface", argbToRgba(colorScheme.inverseOnSurface.toArgb()))
            ret.put("outline", argbToRgba(colorScheme.outline.toArgb()))
        } else {
            ret.put("error", "MaterialYou not supported on this device!")
        }
        invoke.resolve(ret)
    }
    @Command
    fun offsets(invoke: Invoke) {
        val ret = JSObject()
        val window = activity.getWindow()
        val context = activity.getApplication().getApplicationContext()
        val displayMetrics = context.getResources().getDisplayMetrics()
        val controller = ViewCompat.getRootWindowInsets(window.decorView)!!
        val insets = controller.getInsets(WindowInsetsCompat.Type.systemBars())
        ret.put("top", round(insets.top / displayMetrics.density).toInt().toString())
        ret.put("bottom", round(insets.bottom / displayMetrics.density).toInt().toString())
        ret.put("left", round(insets.left / displayMetrics.density).toInt().toString())
        ret.put("right", round(insets.right / displayMetrics.density).toInt().toString())
        invoke.resolve(ret)
    }

    private fun argbToRgba(argb: Int): String {
        val hex = Integer.toHexString(argb)
        return String.format("#%s%s", hex.substring(2), hex.substring(0, 2)).uppercase()
    }

    private fun isUsingNightMode(context: Context): Boolean {
        return when (context.getResources().getConfiguration().uiMode and Configuration.UI_MODE_NIGHT_MASK) {
            Configuration.UI_MODE_NIGHT_YES -> true
            Configuration.UI_MODE_NIGHT_NO -> false
            Configuration.UI_MODE_NIGHT_UNDEFINED -> false
            else -> false
        }
    }
}
