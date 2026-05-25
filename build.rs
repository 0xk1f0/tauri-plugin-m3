const COMMANDS: &[&str] = &["colors", "insets", "bar_color", "contrast"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .build();
}
