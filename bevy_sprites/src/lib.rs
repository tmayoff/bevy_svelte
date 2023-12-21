use bevy::{
    app::{App, PluginGroup},
    window::{Window, WindowPlugin},
    DefaultPlugins,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    gloo_console::log!("Loaded crate");

    let mut app = App::new();
    gloo_console::log!("Created App");

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#bevy_canvas".into()),
            ..Default::default()
        }),
        ..Default::default()
    }));

    gloo_console::log!("Loaded Bevy");
    app.run();
}
