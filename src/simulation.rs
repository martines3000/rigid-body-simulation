use crate::{GlobalState, RenderState};

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct SimulationPlugin;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Cube;

#[derive(Component)]
pub struct Platform;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_startup_system(spawn_platfom)
            .add_system(object_spawner)
            .add_system(object_cleanup)
            .add_system(platform_movement);
    }
}

fn spawn_platfom(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                min_x: -100.0,
                min_y: -1.0,
                min_z: -100.0,
                max_x: 100.0,
                max_y: 1.0,
                max_z: 100.0,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(0.0, 0.0, 1.0, 1.0).into(),
                ..Default::default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(100.0, 1.0, 100.0))
        .insert(Friction::coefficient(0.5))
        .insert(Platform);
}

fn object_spawner(
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    materials: Res<Assets<StandardMaterial>>,
    mut state: ResMut<GlobalState>,
    render_state: Res<RenderState>,
) {
    if state.simulation_state == 0 {
        for _ in 0..state.cube_count {
            commands
                .spawn(PbrBundle {
                    mesh: meshes.get_handle(&render_state.cube_mesh),
                    material: materials.get_handle(&render_state.cube_material),
                    transform: Transform::from_translation(Vec3::new(
                        -50.0 + rand::random::<f32>() * 100.0,
                        20.0 + rand::random::<f32>() * 100.0,
                        -50.0 + rand::random::<f32>() * 100.0,
                    )),
                    ..Default::default()
                })
                .insert(RigidBody::Dynamic)
                .insert(Collider::cuboid(0.5, 0.5, 0.5))
                .insert(Restitution::coefficient(state.restitution_scale))
                .insert(ColliderMassProperties::Density(state.density_scale))
                .insert(Cube);
        }

        for _ in 0..state.ball_count {
            commands
                .spawn(PbrBundle {
                    mesh: meshes.get_handle(&render_state.ball_mesh),
                    material: materials.get_handle(&render_state.ball_material),
                    transform: Transform::from_translation(Vec3::new(
                        -50.0 + rand::random::<f32>() * 100.0,
                        20.0 + rand::random::<f32>() * 100.0,
                        -50.0 + rand::random::<f32>() * 100.0,
                    )),
                    ..Default::default()
                })
                .insert(RigidBody::Dynamic)
                .insert(Collider::ball(0.5))
                .insert(Restitution::coefficient(state.restitution_scale))
                .insert(ColliderMassProperties::Density(state.density_scale))
                .insert(Ball);
        }

        state.simulation_state = 1;
    }
}

fn object_cleanup(
    mut commands: Commands,
    query: Query<Entity, Or<(With<Ball>, With<Cube>)>>,
    mut query_platform: Query<&mut Friction, With<Platform>>,
    mut state: ResMut<GlobalState>,
) {
    if state.simulation_state == 2 {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        for mut friction in query_platform.iter_mut() {
            *friction = Friction::coefficient(state.friction_scale);
        }
        state.simulation_state = 0;
    }
}

// Tilt (rotate) platform with WASD keys
fn platform_movement(
    mut query: Query<&mut Transform, With<Platform>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for mut transform in query.iter_mut() {
        let mut rotation = transform.rotation;
        if keyboard_input.pressed(KeyCode::A) {
            rotation *= Quat::from_rotation_z(0.01);
        }
        if keyboard_input.pressed(KeyCode::D) {
            rotation *= Quat::from_rotation_z(-0.01);
        }
        if keyboard_input.pressed(KeyCode::W) {
            rotation *= Quat::from_rotation_x(0.01);
        }
        if keyboard_input.pressed(KeyCode::S) {
            rotation *= Quat::from_rotation_x(-0.01);
        }
        transform.rotation = rotation;
    }
}
