use rand::Rng;
use amethyst::{
    prelude::*,
    core::{
        transform::Transform,
        math::{
            Vector3,
        },
    },
    assets::{
        AssetStorage,
        Loader,
        Handle,
    },
    renderer::{
        Camera,
        ImageFormat,
        SpriteSheet,
        SpriteRender,
        SpriteSheetFormat,
        Texture,
    },
    window::ScreenDimensions,
};
use crate::boid::Boid;
use crate::boid_system::{
    Matrix2xNf,
};

pub struct Simulator;
impl Simulator {
    fn init_camera(world: &mut World, width: f32, height: f32) {
        let mut transform = Transform::default();
        transform.set_translation_xyz(width * 0.5, height * 0.5, 1.0);

        world.create_entity()
            .with(Camera::standard_2d(width, height))
            .with(transform)
            .build();
    }

    fn init_boids(world: &mut World, sprite_sheet: &Handle<SpriteSheet>, boid_num: usize) {
        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 0,
        };
        let mut transform = Transform::default();
        {
            let fetched = world.try_fetch::<BoidInfo>();
            if let Some(res) = fetched {
                let pos = res.positions.column(boid_num);
                //transform.set_rotation_z_axis(std::f32::consts::PI);
                //transform.set_rotation_z_axis(std::f32::consts::PI / 2.0);
                //transform.set_rotation_z_axis(std::f32::consts::PI / 4.0);
                //transform.append_rotation_z_axis(std::f32::consts::PI / 4.0);
                //transform.rotate_2d(std::f32::consts::PI + std::f32::consts::PI / 4.0);
                transform.set_translation_xyz(pos[0], pos[1], 0.0);
                transform.set_scale(Vector3::new(0.025, 0.035, 0.0));
            }
        }
        world.create_entity()
            .with(sprite_render.clone())
            .with(Boid{})
            .with(transform)
            .build();
    }

    fn load_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
        let loader = world.read_resource::<Loader>();
        let texture_handle = {
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load("sprites/w_triangle.png", ImageFormat::default(), (), &texture_storage)
        };
        let sprite_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load("sprites/w_triangle.ron", SpriteSheetFormat(texture_handle), (), &sprite_storage)
    }
}

impl SimpleState for Simulator {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Simulation start!");
        let nboids = 50;
        let world = data.world;
        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };
        world.insert(BoidInfo::new(nboids, width, 0.0, height, 0.0));

        let sprite_handle = Self::load_spritesheet(world);
        for i in 0..nboids {
            Self::init_boids(world, &sprite_handle, i);
        }
        Self::init_camera(world, width, height);
    }

    fn on_stop(&mut self, _: StateData<'_, GameData<'_, '_>>) {
        println!("Simulation stop!");
    }
}

pub struct BoidInfo {
    pub positions: Matrix2xNf,
    pub velocities: Matrix2xNf,
    pub x_max: f32,
    pub y_max: f32,
    pub x_min: f32,
    pub y_min: f32,
}

impl BoidInfo {
    fn new(nboids: usize, x_max: f32, x_min: f32, y_max: f32, y_min: f32) -> Self {
        BoidInfo {
            //positions: MatrixMN::<f32, U2, U10>::from_fn(|_, _| rand::thread_rng().gen_range(0.0, 250.0)),
            //velocities: MatrixMN::<f32, U2, U10>::zeros(),
            positions: Matrix2xNf::from_fn(nboids, |_, _| rand::thread_rng().gen_range(0.0, 250.0)),
            velocities: Matrix2xNf::zeros(nboids),
            x_max,
            x_min,
            y_max,
            y_min,
        }
    }
}
impl Default for BoidInfo {
    fn default() -> Self {
        BoidInfo {
            //positions: MatrixMN::<f32, U2, U10>::zeros(),
            //velocities: MatrixMN::<f32, U2, U10>::zeros(),
            positions: Matrix2xNf::zeros(1),
            velocities: Matrix2xNf::zeros(1),
            x_max: 0.0,
            y_max: 0.0,
            x_min: 0.0, 
            y_min: 0.0, 
        }
    }
}

