use std::collections::HashMap;

use bevy::prelude::*;

use super::prelude::{BasicCubeId, FaceType};

/**
 * 方块
 */
#[derive(Component)]
pub struct Cube;

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

/**
 * 系统 基础包
 */
pub const BASIC_ID: ModId = ModId(00);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ModId(i32);

impl Default for ModId {
    fn default() -> Self {
        BASIC_ID
    }
}

#[derive(Clone, Copy)]
pub enum FaceTo {
    // 面向
    Z,
    OpsZ,
    X,
    OpsX,
}

/**
 * 默认是指向Z 轴的
 */
impl Default for FaceTo {
    fn default() -> Self {
        Self::Z
    }
}

#[derive(Default, Component, Clone, Copy)]
pub struct CubeData {
    pub mod_id: ModId,
    // 方块的 资源是什么
    pub cube_id: i32,
    // 方块
    pub face_to: FaceTo,
    // 剩余的耐久
    pub retain: f32,
}

//！！ 这里 已经暗示了 通过 CubeData的数据来更新 方块中的数据
// 通过ModId 和 cube_id 来找到 方块里面是 数据张什么样子

/**
 * 获取 地图数据特性
 */
pub trait MapGetter {
    /**
     * 通过一个点 查询 方块的信息
     */
    fn find(&mut self, p: Point3D) -> Option<CubeData>;

    /**
     * 查询一组点 数据
     */
    fn find_list(&mut self, p_list: Vec<Point3D>) -> HashMap<Point3D, CubeData>;
}

/**
 * 判断一个面是否要被加载
 */
pub fn need_to_render<T: MapGetter>(
    map_getter: &mut T,
    point3d: Point3D,
    face_type: FaceType,
) -> bool {
    let check_point: Point3D;
    match face_type {
        FaceType::Up => check_point = Point3D::new(point3d.x, point3d.y + 1, point3d.z),
        FaceType::Down => check_point = Point3D::new(point3d.x, point3d.y - 1, point3d.z),
        FaceType::Forward => check_point = Point3D::new(point3d.x, point3d.y, point3d.z + 1),
        FaceType::Backward => check_point = Point3D::new(point3d.x, point3d.y, point3d.z - 1),
        FaceType::Left => check_point = Point3D::new(point3d.x - 1, point3d.y, point3d.z),
        FaceType::Right => check_point = Point3D::new(point3d.x + 1, point3d.y, point3d.z),
    };
    // 在这个面 的周边只有没有数据 或者 有数据为空的情况下 才不加载
    match map_getter.find(check_point) {
        Some(cube_data) => {
            cube_data.cube_id == (BasicCubeId::EmptyId as i32) && cube_data.mod_id.eq(&BASIC_ID)
        }
        None => true,
    }
}
