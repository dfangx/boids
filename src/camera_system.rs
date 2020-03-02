use amethyst::{
    prelude::*,
    input::{
        InputHandler,
        StringBindings,
    },
    derive::SystemDesc,
    core::{
        transform::Transform,
        SystemDesc,
        timing::Time,
    },
    ecs::prelude::{
        System,
        SystemData,
        WriteStorage,
        ReadStorage,
        Read,
        Join,
    },
    renderer::{
        Camera
    },
};

#[derive(SystemDesc)]
pub struct CameraSystem;
impl<'s> System<'s> for CameraSystem {
    type SystemData = (ReadStorage<'s, Camera>,
                       WriteStorage<'s, Transform>,
                       Read<'s, InputHandler<StringBindings>>,
                       Read<'s, Time>);
    fn run(&mut self, (cameras, mut transforms, input, time): Self::SystemData) {
        for (camera, transform) in (&cameras, &mut transforms).join() {
            let x_axis = input.axis_value("horizontal");
            if let Some(mv_amnt) = x_axis {
                transform.prepend_translation_x((2.0 + time.delta_seconds()) * mv_amnt as f32);
            }

            let y_axis = input.axis_value("vertical");
            if let Some(mv_amnt) = y_axis {
                transform.prepend_translation_y((2.0 + time.delta_seconds()) * mv_amnt as f32);
            }

            let z_axis = input.axis_value("zoom");
            if let Some(mv_amnt) = z_axis {
                transform.prepend_translation_z((1.0 + time.delta_seconds()) * mv_amnt as f32);
            }
            println!("{}", transform.translation())
        }
    }
}
