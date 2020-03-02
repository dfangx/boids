use amethyst::{
    prelude::*,
    core::{
        transform::Transform,
        math::{
            RowVector3,
        },
        timing::Time,
        SystemDesc,
    },
    derive::SystemDesc,
    ecs::prelude::{
        System,
        SystemData,
        Read,
        Write,
        WriteStorage,
        Entities,
    },
};
use crate::{
    boid::Boid,
    resources::{
        WorldDimensions,
        BoidInfo,   
    },
    types::MatrixNx3f,
    quadtree::QuadTree,
};

#[derive(SystemDesc)]
pub struct BoidSystem;
impl BoidSystem {
    fn cohesion(boid_pos: RowVector3<f32>, positions: &MatrixNx3f) -> RowVector3<f32>{
        ((positions.row_sum() - boid_pos)
         .unscale(positions.nrows() as f32 - 1.0) - boid_pos)
            .unscale(100.0)
    }

    fn separation(boid_pos: RowVector3<f32>, positions: &MatrixNx3f, boid_num: usize) -> RowVector3<f32> {
        let mut v_offset = RowVector3::zeros();
        positions.row_iter()
            .enumerate()
            .filter(|(i, x)| {
            *i != boid_num && (x - boid_pos).magnitude_squared() < 1000.0
        })
        .for_each(|(_, x)| v_offset -= x - boid_pos);

        v_offset
    }
    
    fn alignment(boid_vel: RowVector3<f32>, velocities: &MatrixNx3f) -> RowVector3<f32>{
        ((velocities.row_sum() - boid_vel)
         .unscale(velocities.nrows() as f32 - 1.0) - boid_vel)
            .unscale(8.0)
    }

    fn move_to_target(boid_vel: RowVector3<f32>) -> RowVector3<f32> {
        (RowVector3::new(500.0, 500.0, 0.0) - boid_vel) / 100.0
    }
    
    fn limit_area(boid_pos: RowVector3<f32>, x_max: f32, y_max: f32, x_min: f32, y_min: f32) -> RowVector3<f32> {
        let mut v_offset = RowVector3::<f32>::zeros();
        if boid_pos[0] < x_min + 100.0{
            v_offset[0] = 10.0;
        }
        else if boid_pos[0] > x_max - 100.0 {
            v_offset[0] = -10.0;
        }
        
        if boid_pos[1] < y_min + 100.0 {
            v_offset[1] = 10.0;
        }
        else if boid_pos[1] > y_max - 100.0 {
            v_offset[1] = -10.0;
        }
        v_offset
    }
    
    fn limit_velocity(boid_vel: RowVector3<f32>, velocities: &mut MatrixNx3f, boid_num: usize) {
        let vlim = 2500.0;
        let vel = boid_vel.magnitude_squared();
        if vel > vlim {
            velocities.set_row(boid_num, &(boid_vel.unscale(vel) * vlim));
        }
        else {
            velocities.set_row(boid_num, &boid_vel);
        }
    } 
}

impl<'s> System<'s> for BoidSystem {
    type SystemData = (Write<'s, BoidInfo>,
                       WriteStorage<'s, Transform>,
                       WriteStorage<'s, Boid>,
                       Read<'s, WorldDimensions>,
                       Read<'s, Time>,
                       Entities<'s>);
    
    fn run(&mut self, (mut boid_info, mut locals, mut boids, world_dim, time, entities): Self::SystemData) {
        //println!("Old pos matrix: {}", boid_info.positions);
        let mut qt = boid_info.qt.clone();
        
        let mut new_qt = QuadTree::new(world_dim.min_x..world_dim.max_x, world_dim.min_y..world_dim.max_y);
        for world_region in &world_dim.regions {
            let region = qt.search_range(world_region.x_range.clone(), world_region.y_range.clone());
            let nrows = region.len();
            let mut velocities = MatrixNx3f::zeros(nrows);
            let mut positions = MatrixNx3f::zeros(nrows);
            let mut ids = vec![];
            
            for (i, boid) in region.iter().enumerate() {
                let pos = boid.get_position();
                let vel = boid.get_velocity();
                velocities.set_row(i, &vel);
                positions.set_row(i, &pos);
                ids.push(boid.id);
            }
            
            for (i, id) in ids.iter().enumerate() {
                let entity = entities.entity(*id as u32);
                let boid = boids.get_mut(entity).unwrap();
                let transform = locals.get_mut(entity).unwrap();
                
                let mut boid_pos: RowVector3<f32> = positions.row(i).into();
                let boid_vel: RowVector3<f32> = velocities.row(i).into();

                let v1 = Self::cohesion(boid_pos, &positions);
                let v2 = Self::separation(boid_pos, &positions, i);
                let v3 = Self::alignment(boid_vel, &velocities);
                let v4 = Self::limit_area(boid_pos, world_dim.max_x, world_dim.max_y, world_dim.min_x, world_dim.min_y);
                let v5 = Self::move_to_target(boid_pos);

                if nrows == 1 {
                    Self::limit_velocity(boid_vel + v2 + v4, &mut velocities, i); 
                }
                else {
                    Self::limit_velocity(boid_vel + v1 + v2 + v3 + v4, &mut velocities, i); 
                }

                boid_pos += velocities.row(i) * time.delta_seconds();
                positions.set_row(i, &boid_pos);

                let angle = velocities.row(i)[1].atan2(velocities.row(i)[0]) - std::f32::consts::PI / 2.0;

                transform.set_rotation_2d(angle);
                transform.prepend_translation(velocities.row(i).transpose().scale(time.delta_seconds()));

                boid.set_velocity(velocities.row(i).into());
                boid.set_position(boid_pos);
                if !new_qt.insert(boid.clone()) {
                    //println!("{:?}", boid);
                }
            }
        }
        boid_info.qt = new_qt;
    }
}
