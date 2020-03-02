use amethyst::core::math::{
    Vector3,
};
use std::{
    ops::Range,
};

/*
#[derive(Clone, Debug, PartialEq)]
enum QuadChild {
    Point(Vector3<f32>),
    Division(Box<QuadTree>),
}
*/

#[derive(Clone, Debug, PartialEq)]
struct QuadTree {
    points: Vec<Vector3<f32>>,
    northwest: Option<Box<QuadTree>>,
    northeast: Option<Box<QuadTree>>,
    southwest: Option<Box<QuadTree>>,
    southeast: Option<Box<QuadTree>>,
    max_points: usize,
    x_range: Range<f32>,
    y_range: Range<f32>,
    //z_range: Range<f32>,
}

impl QuadTree {
    fn new(x_range: Range<f32>, y_range: Range<f32>) -> Self {
        QuadTree {
            points: vec![],
            northwest: None,
            northeast: None,
            southwest: None,
            southeast: None,
            max_points: 1,
            x_range,
            y_range,
            //z_range,
        }
    }

    fn insert(&mut self, point: Vector3<f32>) -> bool {
        let is_inbounds = self.x_range.contains(&point[0]) && self.y_range.contains(&point[1]);

        if !is_inbounds {
            return false
        }
        
        let is_full = self.points.len() == self.max_points;
        let undivided = self.northwest == None;
        if !is_full && undivided {
            self.points.push(point);
            return true
        }

        if undivided {
            self.subdivide();
            //return false
        }

        if self.northwest.as_mut().unwrap().insert(point) { 
            return true
        }
        if self.northeast.as_mut().unwrap().insert(point) { 
            return true
        }
        if self.southwest.as_mut().unwrap().insert(point) { 
            return true
        }
        if self.southeast.as_mut().unwrap().insert(point) {
            return true
        }
        false
    }

    /*
    fn intersect(&self, x_range: Range<f32>, y_range: Range<f32>) -> bool {
        let left = x_range.start;
        let right = x_range.end;
        let top = y_range.end;
        let bottom = y_range.start;
        if right <= self.x_range.start {
            return false;
        }
        if left <= self.x_range.end {
            return false;
        }
        if top <= self.y_range.start {
            return false;
        }
        if bottom <= self.y_range.end {
            return false;
        }
        
        true
    }
    */

    fn search_node(&self, point: Vector3<f32>) {

    }
    
    fn search_range(&mut self, x_range: Range<f32>, y_range: Range<f32>) -> Vec<Vector3<f32>> {
        let mut results = vec![];
        //let intersect = self.x_range.contains(&x_range.start) || self.x_range.contains(&x_range.end) || self.y_range.contains(&y_range.start) || self.y_range.contains(&y_range.end);
        let intersect = self.x_range.start < x_range.end || self.y_range.start < y_range.end || self.x_range.end > x_range.start || self.y_range.end > y_range.start;
        

        //if self.intersect(x_range.clone(), y_range.clone()) {
        //    return results
        //}
        
        if !intersect {
            return results
        }

        let mut p_drained = self.points.clone();
        p_drained.retain(|point| {
            x_range.contains(&point[0]) && y_range.contains(&point[1])
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

    fn delete() {
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

        for point in &self.points {
            if self.northwest.as_mut().unwrap().insert(*point) { 
                continue;
            }
            if self.northeast.as_mut().unwrap().insert(*point) {
                continue;
            }
            if self.southwest.as_mut().unwrap().insert(*point) {
                continue;
            }
            if self.southeast.as_mut().unwrap().insert(*point) {
                continue;
            }
            return false
        }
        self.points = vec![];
        true
    }
}
