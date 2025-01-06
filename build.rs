const COMMANDS: &[&str] = &["colors", "insets"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .build();
}
