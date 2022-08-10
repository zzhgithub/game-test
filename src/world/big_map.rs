use bevy::prelude::{FromWorld, Vec3};
use rand::prelude::*;

//最小生成高度
const base_height: i32 = 10;
//噪音频率（噪音采样时会用到）
const frequency: f32 = 0.025;
//噪音振幅（噪音采样时会用到）
const amplitude: i32 = 1;

const seed: u64 = 10;

// 方块类型
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    //空
    Nan = 0,
    //泥土
    Dirt = 1,
    //草地
    Grass = 3,
    //碎石
    Gravel = 4,
}

pub struct Chunk {
    // 三层的 数组
    black_list: Vec<Vec<Vec<BlockType>>>,
    //每个Chunk的长宽Size
    width: i32,
    //每个Chunk的高度
    height: i32,
}

impl Chunk {
    fn build() -> Chunk {
        let res = Chunk {
            black_list: Vec::new(),
            width: 30,
            height: 30,
        };
        // 生成 里面的black信息
        res
    }

    // 使用算法初始化 这里的数据
    fn fix_black_list(&self) {
        //
    }
}

// 大地图资源
pub struct BigMap {
    offset0: Vec3,
    offset1: Vec3,
    offset2: Vec3,
    truck_list: Vec<Chunk>,
}

impl BigMap {
    fn init(&mut self) {
        // 创建一个 chunk 并添加进去
        let chunck = Chunk::build();
        self.truck_list.push(chunck);
    }
}

impl FromWorld for BigMap {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        // 初始化大地图资源
        let mut rng = thread_rng();
        let offset0 = Vec3::new(
            rng.gen_range(0..1000) as f32,
            rng.gen_range(0..1000) as f32,
            rng.gen_range(0..1000) as f32,
        );
        let offset1 = Vec3::new(
            rng.gen_range(0..1000) as f32,
            rng.gen_range(0..1000) as f32,
            rng.gen_range(0..1000) as f32,
        );
        let offset2 = Vec3::new(
            rng.gen_range(0..1000) as f32,
            rng.gen_range(0..1000) as f32,
            rng.gen_range(0..1000) as f32,
        );
        let mut res = BigMap {
            offset0,
            offset1,
            offset2,
            truck_list: Vec::new(),
        };
        res.init();
        res
    }
}
