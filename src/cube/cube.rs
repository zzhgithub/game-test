use std::collections::HashMap;

use bevy::prelude::*;

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

pub struct ModId(i32);

impl Default for ModId {
    fn default() -> Self {
        BASIC_ID
    }
}

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

#[derive(Default, Component)]
pub struct CubeData {
    mod_id: ModId,
    // 方块的 资源是什么
    cube_id: i32,
    // 方块
    face_to: FaceTo,
    // 剩余的耐久
    retain: f32,
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
    fn find(p: Point3D) -> Option<CubeData>;

    /**
     * 查询一组点 数据
     */
    fn find_list(p_list: Vec<Point3D>) -> HashMap<Point3D, CubeData>;
}
