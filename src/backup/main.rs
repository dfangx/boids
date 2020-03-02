mod simulator;
mod boid;
mod boid_system;

use simulator::Simulator;
use boid_system::BoidSystem;
use amethyst::{
    prelude::*,
    core::{
        transform::TransformBundle,
    },
    renderer::{
        plugins::{
            RenderFlat2D,
            RenderToWindow,
        },
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};


fn main() -> amethyst::Result<()> {
    //let mut sim = BoidsSim::new();
    //sim.run();
    //sim.print_simstate();
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let display_config = app_root.join("config")
        .join("display.ron");
    let asset_dir = app_root.join("assets");
    let game_data = GameDataBuilder::default()
        .with_bundle(RenderingBundle::<DefaultBackend>::new()
                     .with_plugin(RenderToWindow::from_config_path(display_config)
                                  .with_clear([0.0, 0.0, 0.0, 1.0]))
                     .with_plugin(RenderFlat2D::default()))?
        .with_bundle(TransformBundle::new())?
        .with(BoidSystem, "boid_system", &[]);
    let _world = World::new();
    let mut sim = Application::new(asset_dir, Simulator, game_data)?;
    sim.run();
    Ok(())
}

/*
struct BoidsSim{
    pos_matrix: MatrixMN<f32, U10, U2>,
    vel_matrix: MatrixMN<f32, U10, U2>,
}

impl BoidsSim {
    fn new() -> Self {
        BoidsSim {
            pos_matrix: MatrixMN::<f32, U10, U2>::from_fn(|_, _| rand::thread_rng().gen_range(0.0, 10.0)),
            vel_matrix: MatrixMN::<f32, U10, U2>::zeros(),
        }
    }

    fn run(&mut self) {
        let nboids = self.pos_matrix.nrows();
        for i in 0..3 {
            //println!("{}", self.center_of_mass());
            //println!("{}", self.perceived_velocity());
            self.vel_matrix += self.center_of_mass() + self.perceived_velocity();
            self.pos_matrix += self.vel_matrix;
        }

        //for boid_num in 0..nboids {
            //let offset = self.center_of_mass(boid_num) + self.boid_dist(boid_num) + self.perceived_velocity(boid_num);
            //self.vel_matrix.set_row(boid_num, &offset);
        //}
    }

    fn print_simstate(&self) {
        println!("{}", self.pos_matrix);
        println!("{}", self.vel_matrix);
    }

    fn center_of_mass(&self) -> MatrixMN<f32, U10, U2> {
        let mut row_sums = MatrixMN::<f32, U10, U2>::zeros();
        row_sums.fill_column(0, *self.pos_matrix.row_sum().get(0).unwrap());
        row_sums.fill_column(1, *self.pos_matrix.row_sum().get(1).unwrap());
       
        ((row_sums - self.pos_matrix)
         .unscale(self.pos_matrix.nrows() as f32 - 1.0) - self.pos_matrix)
            .unscale(100.0)
    }
    
    fn boid_dist(&self, boid_num: usize)  -> RowVectorN<f32, U2> {
        let mut dist = RowVectorN::<f32, U2>::zeros();
        let copy_wo_cboid = self.pos_matrix.remove_row(boid_num);
        let cboid = self.pos_matrix.row(boid_num);
        for boid_pos in copy_wo_cboid.row_iter() {
            let bdist = boid_pos - cboid;
            if bdist.magnitude() < 50.0 {
                dist -= bdist
            }
        }
        dist
    }
    
    fn perceived_velocity(&self) -> MatrixMN<f32, U10, U2>{
        let mut row_sums = MatrixMN::<f32, U10, U2>::zeros();
        row_sums.fill_column(0, *self.vel_matrix.row_sum().get(0).unwrap());
        row_sums.fill_column(1, *self.vel_matrix.row_sum().get(1).unwrap());
       
        ((row_sums - self.vel_matrix)
         .unscale(self.vel_matrix.nrows() as f32 - 1.0) - self.vel_matrix)
            .unscale(100.0)
    }
}
*/
