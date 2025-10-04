use godot::prelude::*;
use godot::classes::{IRigidBody2D, RigidBody2D};

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct Apple {
    base: Base<RigidBody2D>
}

#[godot_api]
impl IRigidBody2D for Apple {
    fn init(base: Base<RigidBody2D>) -> Self {
        Apple {
            base
        }
    }
}