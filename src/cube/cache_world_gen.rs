use bevy::utils::HashMap;
use simdnoise::NoiseBuilder;

use super::prelude::{BasicCubeId, CubeData, MapGetter, Point3D, BASIC_ID};

/**
 * 缓存一部分数据的加载器 保证在 一定的情况下会清除缓存
 */
pub struct CacheWorldGen {
    cache: HashMap<Point3D, CubeData>,
    seed: i32,
}

impl CacheWorldGen {
    /**
     * 通过种子创建加载器
     */
    pub fn new(seed: i32) -> Self {
        CacheWorldGen {
            cache: HashMap::new(),
            seed,
        }
    }
    /**
     * 加载区块的数据
     */
    fn load(&mut self, p: Point3D) {
        // 这两个点计算的有问题
        let x_i = if p.x + 15 >= 0 {
            ((p.x + 15) / 30) * 30 - 15
        } else {
            if (p.x + 15) % 30 == 0 {
                ((p.x + 15) / 30) * 30 - 15
            } else {
                ((p.x + 15) / 30) * 30 - 45
            }
        };
        let z_i = if p.z + 15 >= 0 {
            ((p.z + 15) / 30) * 30 - 15
        } else {
            if (p.z + 15) % 30 == 0 {
                ((p.z + 15) / 30) * 30 - 15
            } else {
                ((p.z + 15) / 30) * 30 - 45
            }
        };
        print!("[{:?}]加载核心点[{},{}]", p, x_i, z_i);
        // 算法生成 噪声在赋值给 那个cache
        // 使用算法生成一部分
        // todo 这里可以用 写个 使用name生成的方法
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

        let (noise, min, max) 
        = NoiseBuilder::ridge_2d_offset(x_i as f32, 30, z_i as f32, 30)
            .with_seed(self.seed)
            .with_freq(1.0 / 256.0)
            .with_octaves(5)
            .generate();
        // todo 这里的判断缓存的大小 不够了 清除缓存
        for x in x_i..x_i + 30 {
            for z in z_i..z_i + 30 {
                // print!("{:?}", (x * 30 + z));
                // 噪声加载区块时 平滑！！！
                let mut h = noise[((x - x_i) * 30 + z - z_i) as usize];
                // print!("5-h={:?}", 5.0 - h);
                // print!("(5-h)*256={:?}", (5.0 - h) * 256.0 +60.0);
                h = (h - 4.5) * 1024.0;
                // + 128.0;
                print!("H is {:?}", h);
                for y in 0..=512 {
                    let pp = Point3D::new(x, y, z);
                    if y as f32 > h {
                        self.cache.insert(pp, vess.clone());
                    } else if y > 100 {
                        self.cache.insert(pp, stone.clone());
                    } else if y > 80 {
                        self.cache.insert(pp, grass.clone());
                    } else if y > 60 {
                        self.cache.insert(pp, soil.clone());
                    } else if y >= 0 {
                        self.cache.insert(pp, stone.clone());
                    } else {
                        print!("{:?}", pp);
                    }
                }
            }
        }
        print!("加载完毕");
    }
}

impl MapGetter for CacheWorldGen {
    fn find(&mut self, p: Point3D) -> Option<CubeData> {
        let tmp = CubeData {
            mod_id: BASIC_ID,
            cube_id: BasicCubeId::EmptyId as i32,
            ..Default::default()
        };
        if p.y < 0 || p.y > 512 {
            return Some(tmp);
        }
        match self.cache.get(&p) {
            Some(res) => Some(res.clone()),
            None => {
                // 先根据当前的点 加载区块
                // print!("没有找到的点{:?}", p);
                self.load(p);
                match self.cache.get(&p) {
                    Some(res) => Some(res.clone()),
                    None => Some(tmp),
                }
            }
        }
    }

    fn find_list(&mut self, p_list: Vec<Point3D>) -> std::collections::HashMap<Point3D, CubeData> {
        todo!()
    }
}
