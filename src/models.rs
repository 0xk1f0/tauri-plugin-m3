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
pub struct ColorSchemeError {
    pub error: Option<String>,
}
