use serde::{Serialize, Deserialize};
use crate::model::direction::Direction;
use crate::model::points::{Points, xy_to_point};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}


pub fn shift_point(point: Point, direction: Direction) -> Option<Point> {
    let mut x: i32 = point.x as i32;
    let mut y: i32 = point.y as i32;

    x += direction.h_move();
    y += direction.v_move();

    if x > 7 || x < 0 || y > 7 || y < 0 {
        return None;
    }
    return Some(Point{ x: x as u32, y: y as u32 });
}


pub fn to_point_vec(points: Points) -> Vec<Point> {
    let mut result = vec![];
    for y in 0..=7 {
        for x in 0..=7 {
            if points & xy_to_point(x, y) != 0 {
                result.push(Point{x, y});
            }
        }
    }
    result
}

pub fn to_point(points: Points) -> Point {
    for y in 0..=7 {
        for x in 0..=7 {
            if points & xy_to_point(x, y) != 0 {
                return Point{x, y};
            }
        }
    }
    panic!("Illegal point");
}