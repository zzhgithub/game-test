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

impl Default for BasicCubeId {
    fn default() -> Self {
        Self::EmptyId
    }
}

// Biac 渲染
