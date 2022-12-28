mod simulation;

use std::f32::consts::PI;

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};

use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};

use bevy_egui::{egui, EguiContext, EguiPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(Msaa { samples: 1 })
        .init_resource::<GlobalState>()
        .init_resource::<RenderState>()
        .add_startup_system(setup_scene)
        .add_startup_system(configure_global_state)
        .add_startup_system(load_assets)
        .add_system(bevy::window::close_on_esc)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Rigid body simulation".to_string(),
                width: 500.0,
                height: 500.0,
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            },
            ..default()
        }))
        .add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin::default())
        .add_plugin(EguiPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(simulation::SimulationPlugin)
        .add_system(fps_update_system)
        .add_system(ui)
        .run();
}

fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    // AMBIENT LIGHT
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.75,
    });

    // DIRECTIONAL LIGHT
    // const HALF_SIZE: f32 = 10.0;
    // commands.spawn(DirectionalLightBundle {
    //     directional_light: DirectionalLight {
    //         // Configure the projection to better fit the scene
    //         shadow_projection: OrthographicProjection {
    //             left: -HALF_SIZE,
    //             right: HALF_SIZE,
    //             bottom: -HALF_SIZE,
    //             top: HALF_SIZE,
    //             near: -10.0 * HALF_SIZE,
    //             far: 10.0 * HALF_SIZE,
    //             ..default()
    //         },
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform {
    //         translation: Vec3::new(0.0, 2.0, 0.0),
    //         rotation: Quat::from_rotation_x(-PI / 4.),
    //         ..default()
    //     },
    //     ..default()
    // });

    // FPS TEXT
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 60.0,
                color: Color::GOLD,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..Default::default()
        }),
        FpsText,
    ));

    // ORBIT CAMERA
    commands
        .spawn(Camera3dBundle::default())
        .insert(OrbitCameraBundle::new(
            OrbitCameraController {
                mouse_rotate_sensitivity: Vec2 { x: 0.5, y: 0.5 },
                mouse_translate_sensitivity: Vec2 { x: 0.5, y: 0.5 },
                ..Default::default()
            },
            Vec3::new(0.0, 30.0, 250.0),
            Vec3::new(0., 0., 0.),
        ));
}

#[derive(Default, Resource)]
struct GlobalState {
    simulation_state: usize,
    cube_count: usize,
    ball_count: usize,

    // Forces
    gravity_scale: f32,
    friction_scale: f32,
    restitution_scale: f32,
    density_scale: f32,
}

fn configure_global_state(mut state: ResMut<GlobalState>) {
    state.simulation_state = 0;
    state.cube_count = 1;
    state.ball_count = 1;

    // Forces
    state.gravity_scale = 1.0;
    state.friction_scale = 0.5;
    state.restitution_scale = 0.5;
    state.density_scale = 1.0;
}

#[derive(Default, Resource)]
struct RenderState {
    ball_material: Handle<StandardMaterial>,
    ball_mesh: Handle<Mesh>,
    cube_material: Handle<StandardMaterial>,
    cube_mesh: Handle<Mesh>,
}

fn load_assets(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut render_state: ResMut<RenderState>,
) {
    render_state.ball_material = materials.add(StandardMaterial {
        base_color: Color::rgba(1.0, 0.0, 0.0, 1.0).into(),
        ..Default::default()
    });

    render_state.ball_mesh = meshes.add(Mesh::from(shape::Icosphere {
        radius: 0.5,
        subdivisions: 2,
    }));

    render_state.cube_material = materials.add(StandardMaterial {
        base_color: Color::rgba(0.0, 1.0, 0.0, 1.0).into(),
        ..Default::default()
    });

    render_state.cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
}

fn ui(mut egui_context: ResMut<EguiContext>, mut state: ResMut<GlobalState>) {
    egui::Window::new("Menu")
        .default_size([300.0, 100.0])
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Cube count:");
                ui.add(egui::Slider::new(&mut state.cube_count, 1..=10000));
            });
            ui.horizontal(|ui| {
                ui.label("Ball count:");
                ui.add(egui::Slider::new(&mut state.ball_count, 1..=10000));
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Gravity scale:");
                ui.add(egui::Slider::new(&mut state.gravity_scale, 0.0..=10.0));
            });
            ui.horizontal(|ui| {
                ui.label("Friction scale:");
                ui.add(egui::Slider::new(&mut state.friction_scale, 0.0..=10.0));
            });
            ui.horizontal(|ui| {
                ui.label("Restitution scale:");
                ui.add(egui::Slider::new(&mut state.restitution_scale, 0.0..=10.0));
            });
            ui.horizontal(|ui| {
                ui.label("Density scale:");
                ui.add(egui::Slider::new(&mut state.density_scale, 0.0..=10.0));
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Simulation state:");
                if ui.button("Restart").clicked() {
                    state.simulation_state = 2;
                }
            });
        });
}

#[derive(Component)]
struct FpsText;

fn fps_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}
