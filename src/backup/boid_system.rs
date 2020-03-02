use amethyst::{
    prelude::*,
    core::{
        transform::Transform,
        math::{
            U2,
            Dynamic,
            VecStorage,
            Matrix,
            Vector2,
        },
        timing::Time,
        SystemDesc,
    },
    derive::SystemDesc,
    ecs::prelude::{
        System,
        SystemData,
        Join,
        Read,
        Write,
        WriteStorage,
        ReadStorage,
    },
};
use crate::boid::Boid;
use crate::simulator::BoidInfo;

pub type Matrix2xNf = Matrix<f32, U2, Dynamic, VecStorage<f32, U2, Dynamic>>;
#[derive(SystemDesc)]
pub struct BoidSystem;
impl BoidSystem {
    fn cohesion(boid_pos: Vector2<f32>, positions: &Matrix2xNf) -> Vector2<f32>{
        ((positions.column_sum() - boid_pos)
         .unscale(positions.ncols() as f32 - 1.0) - boid_pos)
            .unscale(100.0)
    }

    fn separation(boid_pos: Vector2<f32>, positions: &Matrix2xNf, boid_num: usize) -> Vector2<f32> {
        let mut v_offset = Vector2::zeros();
        for (i, pos) in positions.column_iter().enumerate() {
            if i != boid_num  {
                let bdist = pos - boid_pos;
                if bdist.magnitude_squared() < 900.0 {
                    v_offset -= bdist;
                }
            }
        }
        v_offset
    }
    fn alignment(boid_vel: Vector2<f32>, velocities: &Matrix2xNf) -> Vector2<f32>{
        ((velocities.column_sum() - boid_vel)
         .unscale(velocities.ncols() as f32 - 1.0) - boid_vel)
            .unscale(8.0)
    }
    
    fn limit_area(boid_pos: Vector2<f32>, x_max: f32, y_max: f32, x_min: f32, y_min: f32) -> Vector2<f32> {
        let mut v_offset = Vector2::<f32>::zeros();
        if boid_pos[0] < x_min {
            v_offset[0] = 10.0;
        }
        else if boid_pos[0] > x_max {
            v_offset[0] = -10.0;
        }
        
        if boid_pos[1] < y_min {
            v_offset[1] = 10.0;
        }
        else if boid_pos[1] > y_max {
            v_offset[1] = -10.0;
        }
        v_offset
    }
    
    fn limit_velocity(boid_vel: Vector2<f32>, velocities: &mut Matrix2xNf, boid_num: usize) {
        let vlim = 2500.0;
        let vel = boid_vel.magnitude_squared();
        if vel > vlim {
            velocities.set_column(boid_num, &(boid_vel.unscale(vel) * vlim));
        }
        else {
            velocities.set_column(boid_num, &boid_vel);
        }
    } 
}

impl<'s> System<'s> for BoidSystem {
    type SystemData = (Write<'s, BoidInfo>,
                       WriteStorage<'s, Transform>,
                       ReadStorage<'s, Boid>,
                       Read<'s, Time>);
    
    fn run(&mut self, (mut boid_info, mut locals, boids, time): Self::SystemData) {
        //println!("Old pos matrix: {}", boid_info.positions);
        for (i, (_, local)) in (&boids, &mut locals).join().enumerate() {
            let mut boid_pos: Vector2<f32> = boid_info.positions.column(i).into();
            let boid_vel: Vector2<f32> = boid_info.velocities.column(i).into();
            
            let v1 = Self::cohesion(boid_pos, &boid_info.positions);
            let v2 = Self::separation(boid_pos, &boid_info.positions, i);
            let v3 = Self::alignment(boid_vel, &boid_info.velocities);
            let v4 = Self::limit_area(boid_pos, boid_info.x_max, boid_info.y_max, boid_info.x_min, boid_info.y_min);
            Self::limit_velocity(boid_vel + v1 + v2 + v3 + v4, &mut boid_info.velocities, i); 
            
            boid_pos += boid_info.velocities.column(i) * time.delta_seconds();
            boid_info.positions.set_column(i, &boid_pos);
            
            let angle = boid_info.velocities.column(i)[1].atan2(boid_info.velocities.column(i)[0]) - std::f32::consts::PI / 2.0;
            
            local.set_rotation_2d(angle);
            local.prepend_translation_x(boid_info.velocities.column(i)[0] * time.delta_seconds());
            local.prepend_translation_y(boid_info.velocities.column(i)[1] * time.delta_seconds());
        }
        //println!("New vel matrix: {}", boid_info.velocities);
        //println!("New pos matrix: {}", boid_info.positions);
    }
}
