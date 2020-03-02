use crate::{
    types::MatrixNx3f,
    quadtree::QuadTree,
};
use std::ops::Range;

#[derive(Default)]
pub struct WorldDimensions {
    pub max_x: f32,
    pub max_y: f32,
    pub min_x: f32,
    pub min_y: f32,
    pub regions: Vec<WorldRegion>,
}

#[derive(Clone)]
pub struct WorldRegion {
    pub x_range: Range<f32>,
    pub y_range: Range<f32>,
}

impl WorldRegion {
    fn new(x_range: Range<f32>, y_range: Range<f32>) -> Self {
        WorldRegion {
            x_range,
            y_range,
        }
    }
}
impl WorldDimensions {
    pub fn new(max_x: f32, min_x: f32, max_y: f32, min_y: f32) -> Self {
        let x_mid = (min_x + max_x) / 2.0;
        let y_mid = (min_y + max_y) / 2.0;
        
        let regions = vec![
            WorldRegion::new(min_x..x_mid, min_y..y_mid),
            WorldRegion::new(x_mid..max_x, min_y..y_mid),
            WorldRegion::new(min_x..x_mid, y_mid..max_y),
            WorldRegion::new(x_mid..max_x, y_mid..max_y),
        ];
        
        WorldDimensions {
            min_x,
            max_x,
            min_y,
            max_y,
            regions,
        }
    }
}

pub struct BoidInfo {
    pub qt: QuadTree,
}

impl BoidInfo {
    pub fn new(x_max: f32, x_min: f32, y_max: f32, y_min: f32) -> Self {
        BoidInfo {
            qt: QuadTree::new(x_min..x_max, y_min..y_max),
        }
    }
}
impl Default for BoidInfo {
    fn default() -> Self {
        BoidInfo {
            qt: QuadTree::new(0.0..1.0, 0.0..1.0),
        }
    }
}

#[derive(Debug)]
pub enum WeatherType {
    Sunny(f32),
    Cloudy(f32),
    Rainy(f32),
}

enum Season {
    Summer,
    Autumn,
    Winter,
    Spring,
}

#[derive(Debug)]
pub struct Weather {
    weather: WeatherType,
    temp: f32,
    pub state_probabilities: [[f32; 3]; 3],
}

impl Default for Weather {
    fn default() -> Self {
        Weather {
            weather: WeatherType::Sunny(0.0),
            temp: 0.0,
            state_probabilities: [
                [0.4, 0.4, 0.2],
                [0.5, 0.4, 0.4],
                [0.1, 0.5, 0.4],
            ],
        }
    }
}

#[derive(PartialEq)]
pub enum CurrentState {
    Running,
    Paused,
}

impl Default for CurrentState {
    fn default() -> Self {
        CurrentState::Paused
    }
}
