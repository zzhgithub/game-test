use std::collections::HashMap;

use bevy::prelude::*;

/**
 * 方块
 */
#[derive(Component)]
pub struct Cube;

/**
 * 接近标识
 */
#[derive(Component)]
pub struct CloseTo(bool);

/**
 * 用来寻找对象的 大型hashmap
 */
#[derive(Default)]
pub struct MapData {
    pub data: HashMap<Point3D, Entity>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Point3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3D {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}
