use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_rapier3d::prelude::*;

use bevy_flycam::PlayerPlugin;
use mycraft::cube::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        // todo 这里要自己去实现 这个可以移动的相机
        // 并且 获取到 这个数据的的值
        // .add_plugin(PlayerPlugin)
        // .add_plugin(EditorPlugin)
        .add_startup_system(setup)
        .run();
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

    render_cube(
        // 这里 borrow进去
        &mut commands,
        quad_handle.clone(),
        texture.clone(),
        Vec3::new(0.0, 0.0, 0.0),
        cuble_size,
    );
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
