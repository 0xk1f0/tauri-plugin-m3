use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorScheme {
    pub primary: Option<String>,
    pub on_primary: Option<String>,
    pub primary_container: Option<String>,
    pub on_primary_container: Option<String>,
    pub inverse_primary: Option<String>,
    pub secondary: Option<String>,
    pub on_secondary: Option<String>,
    pub secondary_container: Option<String>,
    pub on_secondary_container: Option<String>,
    pub tertiary: Option<String>,
    pub on_tertiary: Option<String>,
    pub tertiary_container: Option<String>,
    pub on_tertiary_container: Option<String>,
    pub background: Option<String>,
    pub on_background: Option<String>,
    pub surface: Option<String>,
    pub on_surface: Option<String>,
    pub surface_variant: Option<String>,
    pub on_surface_variant: Option<String>,
    pub inverse_surface: Option<String>,
    pub inverse_on_surface: Option<String>,
    pub outline: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InsetsScheme {
    pub raw_inset_top: Option<u32>,
    pub raw_inset_bottom: Option<u32>,
    pub raw_inset_left: Option<u32>,
    pub raw_inset_right: Option<u32>,
    pub adjusted_inset_top: Option<u32>,
    pub adjusted_inset_bottom: Option<u32>,
    pub adjusted_inset_left: Option<u32>,
    pub adjusted_inset_right: Option<u32>,
    pub scale_factor: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BarColorScheme {
    pub color: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct M3Error {
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct ColorSchemePayload {
    theme: String,
}

#[derive(Serialize)]
pub struct BarColorPayload {
    color: String,
}
