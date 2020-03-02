use std::{
    ops::Range,
};
use crate::boid::Boid;

/*
#[derive(Clone, Debug, PartialEq)]
enum QuadChild {
    Point(Boid),
    Division(Box<QuadTree>),
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct QuadTree {
    boids: Vec<Boid>,
    northwest: Option<Box<QuadTree>>,
    northeast: Option<Box<QuadTree>>,
    southwest: Option<Box<QuadTree>>,
    southeast: Option<Box<QuadTree>>,
    max_boids: usize,
    x_range: Range<f32>,
    y_range: Range<f32>,
    //z_range: Range<f32>,
}

impl QuadTree {
    pub fn new(x_range: Range<f32>, y_range: Range<f32>) -> Self {
        QuadTree {
            boids: vec![],
            northwest: None,
            northeast: None,
            southwest: None,
            southeast: None,
            max_boids: 4,
            x_range,
            y_range,
            //z_range,
        }
    }

    pub fn insert(&mut self, boid: Boid) -> bool {
        let pos = boid.get_position();
        let is_inbounds = self.x_range.contains(&pos[0]) && self.y_range.contains(&pos[1]);

        if !is_inbounds {
            return false
        }
        
        let is_full = self.boids.len() == self.max_boids;
        let undivided = self.northwest == None;
        if !is_full && undivided {
            self.boids.push(boid);
            return true
        }

        if undivided && !self.subdivide() {
            println!("Failed to subdivide");
            return false
        }

        if self.northwest.as_mut().unwrap().insert(boid.clone()) { 
            return true
        }
        if self.northeast.as_mut().unwrap().insert(boid.clone()) { 
            return true
        }
        if self.southwest.as_mut().unwrap().insert(boid.clone()) { 
            return true
        }
        if self.southeast.as_mut().unwrap().insert(boid.clone()) {
            return true
        }
        false
    }

    pub fn search_range(&mut self, x_range: Range<f32>, y_range: Range<f32>) -> Vec<Boid> {
        let mut results = vec![];
        let intersect = self.x_range.start < x_range.end || self.y_range.start < y_range.end || self.x_range.end > x_range.start || self.y_range.end > y_range.start;
        
        if !intersect {
            return results
        }

        let mut p_drained = self.boids.clone();
        p_drained.retain(|boid| {
            let pos = boid.get_position();
            x_range.contains(&pos[0]) && y_range.contains(&pos[1])
        });
        results.append(&mut p_drained);
        
        let undivided = self.northwest == None;
        if undivided {
            return results
        }

        results.append(&mut self.northwest.as_mut().unwrap().search_range(x_range.clone(), y_range.clone()));
        results.append(&mut self.northeast.as_mut().unwrap().search_range(x_range.clone(), y_range.clone()));
        results.append(&mut self.southwest.as_mut().unwrap().search_range(x_range.clone(), y_range.clone()));
        results.append(&mut self.southeast.as_mut().unwrap().search_range(x_range.clone(), y_range.clone()));
        results
    }

    fn subdivide(&mut self) -> bool {
        let x_max = self.x_range.end;
        let y_max = self.y_range.end;
        //let z_max = self.z_range.end();
        let x_min = self.x_range.start;
        let y_min = self.y_range.start;
        //let z_min = self.z_range.start();
        let x_mid = (x_min + x_max) / 2.0;
        let y_mid = (y_min + y_max) / 2.0;
        //let z_mid = z_max / 2.0;

        //println!("x: {}..{}..{} y: {}..{}..{}", x_min, x_mid, x_max, y_min, y_mid, y_max);
        self.northwest = Some(Box::new(QuadTree::new(x_min..x_mid, y_mid..y_max)));
        self.northeast = Some(Box::new(QuadTree::new(x_mid..x_max, y_mid..y_max)));
        self.southwest = Some(Box::new(QuadTree::new(x_min..x_mid, y_min..y_mid)));
        self.southeast = Some(Box::new(QuadTree::new(x_mid..x_max, y_min..y_mid)));

        for boid in &self.boids {
            if self.northwest.as_mut().unwrap().insert(boid.clone()) { 
                continue;
            }
            if self.northeast.as_mut().unwrap().insert(boid.clone()) {
                continue;
            }
            if self.southwest.as_mut().unwrap().insert(boid.clone()) {
                continue;
            }
            if self.southeast.as_mut().unwrap().insert(boid.clone()) {
                continue;
            }
            return false
        }
        self.boids = vec![];
        true
    }
}
