use bevy::utils::HashMap;

use super::{
    cube::MapGetter,
    prelude::{BasicCubeId, CubeData, Point3D, BASIC_ID},
};

pub struct TestGetter {
    data: HashMap<Point3D, CubeData>,
}

impl TestGetter {
    pub fn gen() -> Self {
        let mut res = Self {
            data: HashMap::new(),
        };
        // 使用算法生成一部分
        let grass = CubeData {
            mod_id: BASIC_ID,
            cube_id: BasicCubeId::GrassId as i32,
            ..Default::default()
        };
        let soil = CubeData {
            mod_id: BASIC_ID,
            cube_id: BasicCubeId::SoilId as i32,
            ..Default::default()
        };
        let stone = CubeData {
            mod_id: BASIC_ID,
            cube_id: BasicCubeId::StoneId as i32,
            ..Default::default()
        };
        let vess = CubeData {
            mod_id: BASIC_ID,
            cube_id: BasicCubeId::EmptyId as i32,
            ..Default::default()
        };

        // 随便测试一个初始化 没有任何特别的算法
        // 按照原来的算法 超出边界 会造成崩溃
        for x in -100..100 {
            for y in -100..100 {
                for z in -100..100 {
                    let p = Point3D::new(x, y, z);
                    if y >= 0 && y <= 3 {
                        res.data.insert(p, grass.clone());
                    } else if y >= -8 && y < 0 {
                        res.data.insert(p, soil.clone());
                    } else if y >= -100 && y < 8 {
                        res.data.insert(p, stone.clone());
                    } else {
                        res.data.insert(p, vess.clone());
                    }
                }
            }
        }

        res
    }
}

impl MapGetter for TestGetter {
    fn find(&self, p: Point3D) -> Option<&CubeData> {
        self.data.get(&p)
    }

    fn find_list(&self, p_list: Vec<Point3D>) -> std::collections::HashMap<Point3D, CubeData> {
        todo!()
    }
}
