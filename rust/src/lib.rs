use godot::prelude::*;

mod apple;
mod main_scene;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}