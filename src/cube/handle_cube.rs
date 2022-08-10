// 这里设置 一个方法 可以在某个点就渲染一下数据？
use super::face::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

pub struct SimpleCube {}

/**
 * 方块六面文理
 */
#[derive(Clone)]
pub struct CubeTexture {
    pub forward: Handle<StandardMaterial>,
    pub backward: Handle<StandardMaterial>,
    pub up: Handle<StandardMaterial>,
    pub down: Handle<StandardMaterial>,
    pub left: Handle<StandardMaterial>,
    pub right: Handle<StandardMaterial>,
}

pub fn render_cube(
    commands: &mut Commands,
    quad_handle: Handle<Mesh>,
    texture: CubeTexture,
    vec3: Vec3,
    cube_size: f32,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: quad_handle.clone(),
        material: texture.forward.clone(),
        transform: get_transform_by_face_type_form_vec3(FaceType::Forward, vec3, cube_size),
        ..default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: quad_handle.clone(),
        material: texture.backward.clone(),
        transform: get_transform_by_face_type_form_vec3(FaceType::Backward, vec3, cube_size),
        ..default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: quad_handle.clone(),
        material: texture.up.clone(),
        transform: get_transform_by_face_type_form_vec3(FaceType::Up, vec3, cube_size),
        ..default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: quad_handle.clone(),
        material: texture.down.clone(),
        transform: get_transform_by_face_type_form_vec3(FaceType::Down, vec3, cube_size),
        ..default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: quad_handle.clone(),
        material: texture.left.clone(),
        transform: get_transform_by_face_type_form_vec3(FaceType::Left, vec3, cube_size),
        ..default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: quad_handle.clone(),
        material: texture.right.clone(),
        transform: get_transform_by_face_type_form_vec3(FaceType::Right, vec3, cube_size),
        ..default()
    });
    // 绑定一个碰撞体
    commands
        .spawn()
        .insert(Collider::cuboid(
            cube_size / 2.0,
            cube_size / 2.0,
            cube_size / 2.0,
        ))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            vec3.x, vec3.y, vec3.z,
        )));
}
