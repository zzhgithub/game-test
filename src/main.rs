use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use std::collections::HashMap;

use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::crossbeam::epoch::Pointable};

use bevy_flycam::{FlyCam, PlayerPlugin};
use mycraft::cube::prelude::*;

const load_size: f32 = 30.0;
const check_size: f32 = 15.0;

fn main() {
    App::new()
        .insert_resource(TextureMap {
            data: HashMap::new(),
        })
        // 这个资源只是mapData的缓存
        .insert_resource(TestGetter::gen())
        .insert_resource(MapData {
            data: HashMap::new(),
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        // todo 这里要自己去实现 这个可以移动的相机
        // 并且 获取到 这个数据的的值
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup)
        .add_system(dynamic_load_system)
        .add_system(dynamic_render_system)
        .run();
}

/**
 * todo 动态绘制系统
 * 当 cube added的时候 和 cube 的状态改变时 才重新 绘制！
 * 注意这是一段测试代码
 * 当单个的 数据发生了变化的时候 也要更新！！！
 */
fn dynamic_render_system(
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    mut test_getter: ResMut<TestGetter>,
    mut materials_map: ResMut<TextureMap>,
    query: Query<(Entity, &Transform, &CubeData), Added<CubeData>>,
) {
    let test_map = test_getter.as_mut();
    let mut m_map = &mut materials_map.as_mut().data;

    const CUBLE_SIZE: f32 = 1.0;
    // 声明一个 2D 的贴图
    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        CUBLE_SIZE, CUBLE_SIZE,
    ))));

    // 查询新增的 组件
    for (ele, transform, cube_data) in query.iter() {
        // 这里 是测试代码 如果组件的 y 小于等于0就进行渲染
        // todo 这里优化面的加载
        if cube_data.cube_id != BasicCubeId::EmptyId as i32 {
            // 不是空 才进行处理
            let check_point = Point3D::new(
                transform.translation.x as i32,
                transform.translation.y as i32,
                transform.translation.z as i32,
            );
            commands.entity(ele).add_children(|childern| {
                // info!("判断一个是否加载{}",need_to_render(test_map,check_point,FaceType::Up));
                if need_to_render(test_map, check_point, FaceType::Up) {
                    childern.spawn_bundle(PbrBundle {
                        mesh: quad_handle.clone(),
                        material: getMaterial(cube_data.clone(), FaceType::Up, m_map.to_owned()),
                        transform: get_transform_by_face_type(
                            FaceType::Up,
                            Transform::from_xyz(0.0, 0.0, 0.0),
                            CUBLE_SIZE,
                        ),
                        ..Default::default()
                    });
                }
                if need_to_render(test_map, check_point, FaceType::Down) {
                    childern.spawn_bundle(PbrBundle {
                        mesh: quad_handle.clone(),
                        material: getMaterial(cube_data.clone(), FaceType::Down, m_map.to_owned()),
                        transform: get_transform_by_face_type(
                            FaceType::Down,
                            Transform::from_xyz(0.0, 0.0, 0.0),
                            CUBLE_SIZE,
                        ),
                        ..Default::default()
                    });
                }
                if need_to_render(test_map, check_point, FaceType::Forward) {
                    childern.spawn_bundle(PbrBundle {
                        mesh: quad_handle.clone(),
                        material: getMaterial(
                            cube_data.clone(),
                            FaceType::Forward,
                            m_map.to_owned(),
                        ),
                        transform: get_transform_by_face_type(
                            FaceType::Forward,
                            Transform::from_xyz(0.0, 0.0, 0.0),
                            CUBLE_SIZE,
                        ),
                        ..Default::default()
                    });
                }
                if need_to_render(test_map, check_point, FaceType::Backward) {
                    childern.spawn_bundle(PbrBundle {
                        mesh: quad_handle.clone(),
                        material: getMaterial(
                            cube_data.clone(),
                            FaceType::Backward,
                            m_map.to_owned(),
                        ),
                        transform: get_transform_by_face_type(
                            FaceType::Backward,
                            Transform::from_xyz(0.0, 0.0, 0.0),
                            CUBLE_SIZE,
                        ),
                        ..Default::default()
                    });
                }
                if need_to_render(test_map, check_point, FaceType::Right) {
                    childern.spawn_bundle(PbrBundle {
                        mesh: quad_handle.clone(),
                        material: getMaterial(cube_data.clone(), FaceType::Right, m_map.to_owned()),
                        transform: get_transform_by_face_type(
                            FaceType::Right,
                            Transform::from_xyz(0.0, 0.0, 0.0),
                            CUBLE_SIZE,
                        ),
                        ..Default::default()
                    });
                }
                if need_to_render(test_map, check_point, FaceType::Left) {
                    childern.spawn_bundle(PbrBundle {
                        mesh: quad_handle.clone(),
                        material: getMaterial(cube_data.clone(), FaceType::Left, m_map.to_owned()),
                        transform: get_transform_by_face_type(
                            FaceType::Left,
                            Transform::from_xyz(0.0, 0.0, 0.0),
                            CUBLE_SIZE,
                        ),
                        ..Default::default()
                    });
                }
            });
        }
    }
}

/**
 * 动态加载数据
 */
fn dynamic_load_system(
    mut mapdata: ResMut<MapData>,
    mut commands: Commands,
    mut query: Query<&mut Transform, With<FlyCam>>,
    mut test_getter: ResMut<TestGetter>,
) {
    let test_map = test_getter.as_mut();
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

        // todo 八个方向 中 如果发现 5格子内没有了才进行加载
        // fixme 先处理 要不加载的数据 可以先进行清除? 全部清除掉 其中有的 CloseTo 组件？
        if !data.contains_key(&Point3D::new(may_x as i32, may_y as i32, may_z as i32))
            || !data.contains_key(&Point3D::new(
                (may_x + check_size) as i32,
                may_y as i32,
                may_z as i32,
            ))
            || !data.contains_key(&Point3D::new(
                (may_x - check_size) as i32,
                may_y as i32,
                may_z as i32,
            ))
            || !data.contains_key(&Point3D::new(
                may_x as i32,
                (may_y + check_size) as i32,
                may_z as i32,
            ))
            || !data.contains_key(&Point3D::new(
                may_x as i32,
                (may_y - check_size) as i32,
                may_z as i32,
            ))
            || !data.contains_key(&Point3D::new(
                may_x as i32,
                may_y as i32,
                (may_z + check_size) as i32,
            ))
            || !data.contains_key(&Point3D::new(
                may_x as i32,
                may_y as i32,
                (may_z - check_size) as i32,
            ))
        {
            info!("clean");
            // 查询过 远的数据 并且清除
            let mut to_remove = Vec::new();
            for (point3d, entity) in &*data {
                let sum = (may_x - point3d.x as f32).powi(2)
                    + (may_y - point3d.y as f32).powi(2)
                    + (may_z - point3d.z as f32).powi(2);
                if sum.sqrt() > ((load_size as f32).powi(2) * 2.0 as f32).sqrt() {
                    to_remove.push(point3d.to_owned());
                    commands.entity(entity.to_owned()).despawn_recursive();
                }
            }
            for key in to_remove.iter() {
                data.remove(key);
            }

            info!("loading");
            // ! 加载数据
            for x in (may_x - load_size) as i32..(may_x + load_size) as i32 {
                for y in (may_y - load_size) as i32..(may_y + load_size) as i32 {
                    for z in (may_z - load_size) as i32..(may_z + load_size) as i32 {
                        let check_point = Point3D::new(x, y, z);
                        let entity;
                        match data.get(&check_point) {
                            Some(e) => {
                                entity = e.to_owned();
                                // info!("已经加载了{:?}", check_point);
                            }
                            None => {
                                // todo 这里要判断一下 是否可以取到？ 这里要判断一下这里的 block 是否要进入到这里面
                                // 如果不存在的情况下 创建这个 对象
                                match test_map.find(check_point) {
                                    Some(cube_data) => {
                                        // info!("正在加载{:?}", check_point);
                                        entity = commands
                                            .spawn_bundle(SpatialBundle {
                                                visibility: Visibility { is_visible: true },
                                                transform: Transform::from_xyz(
                                                    x as f32, y as f32, z as f32,
                                                ),
                                                ..Default::default()
                                            })
                                            .insert(Cube)
                                            .insert(cube_data.clone())
                                            .id();
                                        data.insert(check_point, entity);
                                    }
                                    None => {
                                        info!("{:?} 没有找到cube数据", check_point)
                                    }
                                }
                                // 把这个新的 缓存进去

                                // 这里 应该根据 某种方法来判断 里面的值? 通过一个点 来加载里面的信息
                                // todo load_cube_by_point3d 方法查询 这个点 应该算出来的结果~~

                                // 然后添加一个偏移量
                            }
                        }
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
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut materials_map: ResMut<TextureMap>,
) {
    let mut m_map = &mut materials_map.as_mut().data;
    // 加载大地图是在 别的地方做！！！
    // 尝试展示 一个面
    // 加载资源
    // 加载地图资源 然后在更新系统中 渲染具体的数据

    // let texture_handle: Handle<Image> = asset_server.load("a.jpeg");
    // 使用图片生成一种文理
    // let material_handle = materials.add(StandardMaterial {
    //     base_color_texture: Some(texture_handle.clone()),
    //     alpha_mode: AlphaMode::Blend,
    //     unlit: true,
    //     ..default()
    // });

    // todo 这里初始化加载 默认的资源

    // 遍历枚举中的值
    for i in BasicCubeId::ALL {
        for w in FaceType::ALL {
            let key = i.to_string() + "-" + w;
            let path = "basic/".to_owned() + &i.to_string() + "/" + w + ".png";
            let texture_handle: Handle<Image> = asset_server.load(&path);
            let material_handle = materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle.clone()),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            });
            m_map.insert(key, material_handle);
        }
    }
    info!("材质加载完毕");
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 5.0, -4.0),
        ..default()
    });
}
