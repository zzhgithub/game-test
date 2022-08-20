use bevy::utils::HashMap;
use simdnoise::NoiseBuilder;

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

        let SIZE_TUNCK = 100;

        let noise = NoiseBuilder::ridge_2d_offset(0.0, SIZE_TUNCK, 0.0, SIZE_TUNCK)
            .with_seed(19092)
            .with_freq(1.0 / 256.0)
            .with_octaves(5)
            .generate_scaled(30.0, 60.0);

        // 通过噪声 生成 地形
        for x in 0..SIZE_TUNCK {
            for z in 0..SIZE_TUNCK {
                let h = noise[x * SIZE_TUNCK + z];
                // let check_h = (h - 4.5) * 1024.0 + 128.0;
                // print!("检查的高度[{:?}] ", h);
                // print!("检查的高度[{:?}]", check_h);
                for y in 0..=256 {
                    let p = Point3D::new(x as i32, y as i32, z as i32);
                    if (y as f32) < h {
                        res.data.insert(p, grass.clone());
                    }
                }
            }
            // println!("")
        }

        res
    }
}

impl MapGetter for TestGetter {
    fn find(&self, p: Point3D) -> Option<CubeData> {
        let tmp = CubeData {
            mod_id: BASIC_ID,
            cube_id: BasicCubeId::EmptyId as i32,
            ..Default::default()
        };
        match self.data.get(&p) {
            Some(rs) => Some(rs.clone()),
            None => Some(tmp),
        }
    }

    fn find_list(&self, p_list: Vec<Point3D>) -> std::collections::HashMap<Point3D, CubeData> {
        todo!()
    }
}
