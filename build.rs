const COMMANDS: &[&str] = &["colors", "offsets"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .build();
}
