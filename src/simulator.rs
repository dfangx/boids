use rand::Rng;
use amethyst::{
    prelude::*,
    core::{
        transform::Transform,
        math::{
            Vector3,
        },
        timing::Time,
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
use crate::{
    boid::Boid,
    resources::{
        BoidInfo,
        WorldDimensions,
        CurrentState,
    },
};

pub struct Simulator;
impl Simulator {
    fn init_camera(world: &mut World, width: f32, height: f32) {
        let mut transform = Transform::default();
        transform.set_translation_xyz(width * 0.5, height * 0.5, 550.0);
        world.create_entity()
            .with(Camera::standard_3d(width, height))
            .with(transform)
            .build();
    }

    fn init_boids(world: &mut World, sprite_sheet: &Handle<SpriteSheet>, boid_num: usize) {
        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 0,
        };
        let mut transform = Transform::default();
        let mut rng = rand::thread_rng();
        let position = Vector3::new(
            if boid_num >= 50 {
                rng.gen_range(350.0, 450.0)
            }
            else {
                rng.gen_range(50.0, 150.0)
            }, rng.gen_range(0.0, 50.0), 0.0);
        transform.set_translation(position);
        transform.set_scale(Vector3::new(0.025, 0.035, 0.0));

        let mut boid = Boid::new(position.transpose(), boid_num);
        //boid.set_id(boid_num);
        let entity = world.create_entity()
            .with(sprite_render)
            .with(boid.clone())
            .with(transform)
            .build();
        world.write_component::<Boid>().get_mut(entity).unwrap().set_id(entity.id() as usize);
        {
            let fetched = world.try_fetch_mut::<BoidInfo>();
            boid.set_id(entity.id() as usize);
            if let Some(mut res) = fetched {
                res.qt.insert(boid);
            };
        }
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
        let nboids = 100;
        let world = data.world;
        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };
        println!("{}x{}", width, height);
        world.insert(WorldDimensions::new(width, 0.0, height, 0.0));
        world.insert(BoidInfo::new(width, 0.0, height, 0.0));

        let sprite_handle = Self::load_spritesheet(world);
        for i in 0..nboids {
            Self::init_boids(world, &sprite_handle, i);
        }
        Self::init_camera(world, width, height);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if data.world.read_resource::<Time>().absolute_time_seconds() % 15.0 < 0.01 {
            *data.world.write_resource::<CurrentState>() = CurrentState::Running;
        }
        else {
            *data.world.write_resource::<CurrentState>() = CurrentState::Paused;
        }
        Trans::None
    }
}


