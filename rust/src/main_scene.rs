use godot::prelude::*;
// use godot::classes::{Node, INode};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    base: Base<Node>
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Node>) -> Self {
        Main {
            base
        }
    }
}