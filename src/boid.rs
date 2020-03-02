use amethyst::{
    ecs::prelude::{
        Component,
        VecStorage,
    },
    core::math::RowVector3,
};

#[derive(PartialEq, Clone, Debug)]
pub struct Boid {
    pub id: usize,
    position: RowVector3<f32>,
    velocity: RowVector3<f32>,
}

impl Boid {
    pub fn new(position: RowVector3<f32>, id: usize) -> Self {
        Boid {
            id,
            position,
            velocity: RowVector3::zeros(),
        }
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    pub fn set_position(&mut self, position: RowVector3<f32>) {
        self.position = position;
    }

    pub fn set_velocity(&mut self, velocity: RowVector3<f32>) {
        self.velocity = velocity;
    }
    
    pub fn get_position(&self) -> RowVector3<f32> {
        self.position
    }

    pub fn get_velocity(&self) -> RowVector3<f32> {
        self.velocity
    }
}

impl Component for Boid {
    type Storage = VecStorage<Self>;
}

