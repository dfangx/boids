use amethyst::{
    ecs::prelude::{
        Component,
        VecStorage,
    },
};

#[derive(PartialEq, Clone)]
pub struct Boid {
    position: Vector2<f32>,
    velocity: Vector2<f32>,
}

impl Component for Boid {
    type Storage = VecStorage<Self>;
}

