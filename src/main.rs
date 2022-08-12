use std::collections::HashMap;

use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_rapier3d::prelude::*;

use bevy_flycam::{FlyCam, PlayerPlugin};
use mycraft::cube::prelude::*;

fn main() {
    App::new()
        .insert_resource(MapData {
            data: HashMap::new(),
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        // todo 这里要自己去实现 这个可以移动的相机
        // 并且 获取到 这个数据的的值
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup)
        .add_system(dynamic_load_system)
        .run();
}

/**
 * todo 动态绘制系统
 * 当 cube added的时候 和 cube 的状态改变时 才重新 绘制！
 */
fn dynamic_render_system() {}

/**
 * 动态加载数据
 */
fn dynamic_load_system(
    mut mapdata: ResMut<MapData>,
    mut commands: Commands,
    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    // 查询到主相机的 变化
    for transform in query.iter() {
        // 这里 要判断 加载到系统的值吗？
        let data = &mut mapdata.as_mut().data;
        // 获取当 的点 大致属于那个单元格子
        let at = transform.translation;
        let may_x = at.x.round();
        let may_y = at.y.round();
        let may_z = at.z.round();
        // 数据每一帧都在加载 是不行的
        // 必选它走到边缘的时候才加载！！！
        let load_size: f32 = 15.0;
        let check_size: f32 = 5.0;
        // todo 八个方向 中 如果发现 5格子内没有了才进行加载
        // fixme 先处理 要不加载的数据 可以先进行清除? 全部清除掉 其中有的 CloseTo 组件？
        if !data.contains_key(&Point3D::new(may_x as i32, may_y as i32, may_z as i32)) {
            // ! 加载数据
            for x in (may_x - load_size) as i32..(may_x + load_size) as i32 {
                for y in (may_y - load_size) as i32..(may_y + load_size) as i32 {
                    for z in (may_z - load_size) as i32..(may_z + load_size) as i32 {
                        let check_point = Point3D::new(x, y, z);
                        let entity;
                        match data.get(&check_point) {
                            Some(e) => {
                                entity = e.to_owned();
                                info!("已经加载了{:?}", check_point);
                            }
                            None => {
                                // 这里要判断一下这里的 block 是否要进入到这里面
                                // !!
                                // 如果不存在的情况下 创建这个 对象
                                entity = commands.spawn().insert(Cube).id();
                                // 把这个新的 缓存进去
                                data.insert(check_point, entity);
                                // 这里 应该根据 某种方法来判断 里面的值? 通过一个点 来加载里面的信息
                                // todo load_cube_by_point3d 方法查询 这个点 应该算出来的结果~~
                                info!("正在加载{:?}", check_point);
                                // 然后添加一个偏移量
                            }
                        }
                        // 然后在对这个对象 进行处理
                    }
                }
            }
        }
    }
    // let may_at = Vec3::new(at.x.round(), at.y.round(), at.z.round());
}

/**
 * 设置初始化系统
 */
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 加载大地图是在 别的地方做！！！
    // 尝试展示 一个面
    // 加载资源
    // 加载地图资源 然后在更新系统中 渲染具体的数据

    const cuble_size: f32 = 1.0;
    let texture_handle: Handle<Image> = asset_server.load("a.jpeg");
    // 声明一个 2D 的贴图
    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        cuble_size, cuble_size,
    ))));
    // 使用图片生成一种文理
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    let texture = CubeTexture {
        forward: material_handle.clone(),
        backward: material_handle.clone(),
        up: material_handle.clone(),
        down: material_handle.clone(),
        left: material_handle.clone(),
        right: material_handle.clone(),
    };

    // render_cube(
    //     // 这里 borrow进去
    //     &mut commands,
    //     quad_handle.clone(),
    //     texture.clone(),
    //     Vec3::new(0.0, 0.0, 0.0),
    //     cuble_size,
    // );

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 5.0, -4.0),
        ..default()
    });

    // 怎么生成一个可以漫游的相机

    // 生成一个第一视角

    // 这里测试 暂时使用 一个包里的漫游相机

    // camera
    // commands.spawn_bundle(Camera3dBundle {
    //     transform: Transform::from_xyz(3.0, 7.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });
}
