use std::collections::HashMap;

use bevy::prelude::{Handle, StandardMaterial};

use super::prelude::{CubeData, FaceType};

#[derive(Clone, Copy)]
pub enum BasicCubeId {
    // 虚空
    EmptyId = 0,
    // 草
    GrassId = 1,
    // 土壤
    SoilId = 2,
    // 石头
    StoneId = 3,
}
impl BasicCubeId {
    pub const ALL: [i32; 3] = [1, 2, 3];
}

impl Default for BasicCubeId {
    fn default() -> Self {
        Self::EmptyId
    }
}

// Biac 渲染

#[derive(Default, Clone)]
pub struct TextureMap {
    pub data: HashMap<String, Handle<StandardMaterial>>,
}

/**
 * 获取贴图资源
 */
pub fn getMaterial(
    cube_data: CubeData,
    face_type: FaceType,
    map: HashMap<String, Handle<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    let key = cube_data.cube_id.to_string() + "-" + face_type.to_string();
    match map.get(&key) {
        Some(ele) => ele.clone(),
        None => panic!("找到正常的"),
    }
}
