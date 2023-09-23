use godot::engine::{AnimatedSprite2D, CollisionShape2D, Node, NodeVirtual, PhysicsBody2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct TestHotReloading {
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl NodeVirtual for TestHotReloading {
    fn init(base: Base<Node>) -> Self {
        TestHotReloading { base }
    }
}
