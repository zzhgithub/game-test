use bevy::prelude::*;
use std::f32::consts::PI;

/**
 * 朝向面类型
 */
pub enum FaceType {
    // 前
    Forward,
    // 后
    Backward,
    // 上
    Up,
    // 下
    Down,
    // 左
    Left,
    // 右
    Right,
}

// 通过 Vec3获取 面的变换
pub fn get_transform_by_face_type_form_vec3(
    face_type: FaceType,
    vec3: Vec3,
    cube_size: f32,
) -> Transform {
    get_transform_by_face_type(
        face_type,
        Transform::from_xyz(vec3.x, vec3.y, vec3.z),
        cube_size,
    )
}

// 获取面变换
pub fn get_transform_by_face_type(
    face_type: FaceType,
    transform: Transform,
    cube_size: f32,
) -> Transform {
    let tans = transform.translation;
    match face_type {
        FaceType::Forward => Transform {
            translation: Vec3::new(tans.x, tans.y, tans.z + cube_size / 2.0),
            ..default()
        },
        FaceType::Backward => Transform {
            translation: Vec3::new(tans.x, tans.y, tans.z - cube_size / 2.0),
            rotation: Quat::from_rotation_y(PI),
            ..default()
        },
        FaceType::Up => Transform {
            translation: Vec3::new(tans.x, tans.y + cube_size / 2.0, tans.z),
            rotation: Quat::from_rotation_x(-PI / 2.0),
            ..default()
        },
        FaceType::Down => Transform {
            translation: Vec3::new(tans.x, tans.y, tans.y + cube_size / 2.0),
            rotation: Quat::from_rotation_x(PI / 2.0),
            ..default()
        },
        FaceType::Left => Transform {
            translation: Vec3::new(tans.x - cube_size / 2.0, tans.y, tans.z),
            rotation: Quat::from_rotation_y(-PI / 2.0),
            ..default()
        },
        FaceType::Right => Transform {
            translation: Vec3::new(tans.x + cube_size / 2.0, tans.y, tans.z),
            rotation: Quat::from_rotation_y(PI / 2.0),
            ..default()
        },
    }
}
