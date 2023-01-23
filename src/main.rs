use std::f32::consts::PI;

use bevy::prelude::*;
// use bevy_editor_pls::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_infinite_grid::{GridShadowCamera, InfiniteGrid, InfiniteGridBundle, InfiniteGridPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_obj::*;
use iyes_loopless::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(InfiniteGridPlugin)
        // .add_plugin(EditorPlugin)
        .add_plugin(ObjPlugin)
        .add_startup_system(setup)
        .add_plugin(EguiPlugin)
        .add_system(egui_system)
        .insert_resource(IsRunning {
            sy: false,
            sz: false,
            ez: false,
            wz: false,
            wy: false,
        })
        .add_system(rotate.run_if(is_running))
        .add_plugin(WorldInspectorPlugin)
        .run();
}

#[derive(Component)]
struct AngleRB(i32);

#[derive(Component)]
struct AngleRA(i32);

#[derive(Debug, Component, PartialOrd, Ord, PartialEq, Eq)]
enum Rotor {
    Sy,
    Sz,
    Ez,
    Wz,
    Wy,
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut ass: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(InfiniteGridBundle {
        grid: InfiniteGrid {
            // shadow_color: None,
            ..Default::default()
        },
        ..Default::default()
    });
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // Shoulder y
    let a = commands
        .spawn((
            PbrBundle {
                mesh: ass.load("link1g.obj"),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(0., 0.3, 0.),
                ..default()
            },
            AngleRB(0),
            AngleRA(0),
            Rotor::Sy,
        ))
        .id();

    // Shoulder z
    let b = commands
        .spawn((
            PbrBundle {
                mesh: ass.load("link2g2 (2).obj"),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform {
                    translation: Vec3 {
                        x: 0.0,
                        y: 1.5,
                        z: 0.0,
                    },
                    scale: Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    ..default()
                },
                ..default()
            },
            AngleRB(0),
            AngleRA(0),
            Rotor::Sz,
        ))
        .id();

    // Elbow z
    let c = commands
        .spawn((
            PbrBundle {
                mesh: ass.load("link3g.obj"),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform {
                    translation: Vec3 {
                        x: 0.0,
                        y: 2.25,
                        z: 0.0,
                    },
                    scale: Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    ..default()
                },
                ..default()
            },
            AngleRB(0),
            AngleRA(0),
            Rotor::Ez,
        ))
        .id();

    // Wrist z
    let d = commands
        .spawn((
            PbrBundle {
                mesh: ass.load("link4g.obj"),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform {
                    translation: Vec3 {
                        x: 0.0,
                        y: 2.,
                        z: 0.0,
                    },
                    scale: Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    ..default()
                },
                ..default()
            },
            AngleRB(0),
            AngleRA(0),
            Rotor::Wz,
        ))
        .id();

    // wrist y
    let e = commands
        .spawn((
            PbrBundle {
                mesh: ass.load("link5g.obj"),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform {
                    translation: Vec3 {
                        x: 0.0,
                        y: 1.15,
                        z: 0.0,
                    },
                    scale: Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    ..default()
                },
                ..default()
            },
            AngleRB(0),
            AngleRA(0),
            Rotor::Wy,
        ))
        .id();

    commands.entity(d).push_children(&[e]);
    commands.entity(c).push_children(&[d]);
    commands.entity(b).push_children(&[c]);
    commands.entity(a).push_children(&[b]);

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 10., 10.).looking_at(
            Vec3 {
                x: 0.0,
                y: 4.,
                z: 0.0,
            },
            Vec3::Y,
        ),
        ..default()
    });
}

fn egui_system(
    mut egui_context: ResMut<EguiContext>,
    isr: ResMut<IsRunning>,
    mut query: Query<(&mut Transform, &mut AngleRB, &mut AngleRA, &Rotor)>,
) {
    egui::Window::new("Arm Control").show(egui_context.ctx_mut(), |ui| {
        let mut v: Vec<_> = query.iter_mut().collect();
        v.sort_by(|a, b| a.3.cmp(b.3));
        for (t, b, mut a, r) in v {
            ui.add(egui::Slider::new(&mut a.0, -170..=170));
            
        }
        if ui.button("run").clicked() {
            run_simulation(isr, query);
        }
    });
}

fn run_simulation(
    mut isr: ResMut<IsRunning>,
    mut query: Query<(&mut Transform, &mut AngleRB, &mut AngleRA, &Rotor)>,
) {
    debug!("Simulation started!");
    isr.sy = true;
    isr.sz = true;
    isr.ez = true;
    isr.wz = true;
    isr.wy = true;

    for (mut t, mut b, a, r) in query.iter_mut() {
        t.rotation = Quat::IDENTITY;
        b.0 = 0;
    }
}

#[derive(Default, Resource)]
struct IsRunning {
    sy: bool,
    sz: bool,
    ez: bool,
    wz: bool,
    wy: bool,
}

impl IsRunning {
    fn is_running(&self) -> bool {
        // self.sy || self.sz || self.ez || self.wz || self.wy
        true
    }
}

fn is_running(isr: Res<IsRunning>) -> bool {
    isr.is_running()
}

fn rotate(
    time: Res<Time>,
    mut isr: ResMut<IsRunning>,
    mut query: Query<(&mut Transform, &mut AngleRB, &mut AngleRA, &Rotor)>,
) {
    // println!("");
    // println!("");
    for (mut t, mut b, a, r) in query.iter_mut() {
        // РАБОТАЕТ
        match r {
            Rotor::Sy => {
                t.rotate_y(((a.0 - b.0).signum() as f32).to_radians());
                // println!("{:?} {:?}", r, t.rotation.to_axis_angle());
                b.0 = (b.0 + (a.0 - b.0).signum());
                if b.0 == a.0 {
                    isr.sy = false
                }
            }
            Rotor::Sz => {
                t.rotate_z(((a.0 - b.0).signum() as f32).to_radians());
                // println!("{:?} {:?}", r, t.rotation.to_axis_angle());
                b.0 += (a.0 - b.0).signum();
                if b.0 == a.0 {
                    isr.sz = false
                }
            }
            Rotor::Ez => {
                t.rotate_z(((a.0 - b.0).signum() as f32).to_radians());
                // println!("{:?} {:?}", r, t.rotation.to_axis_angle());
                b.0 += (a.0 - b.0).signum();
                if b.0 == a.0 {
                    isr.ez = false
                }
            }
            Rotor::Wz => {
                t.rotate_z(((a.0 - b.0).signum() as f32).to_radians());
                // println!("{:?} {:?}", r, t.rotation.to_axis_angle());
                b.0 = (b.0 + (a.0 - b.0).signum());
                if b.0 == a.0 {
                    isr.wz = false
                }
            }
            Rotor::Wy => {
                t.rotate_y(((a.0 - b.0).signum() as f32).to_radians());
                // println!("{:?} {:?}", r, t.rotation.to_axis_angle());
                b.0 += (a.0 - b.0).signum();
                if b.0 == a.0 {
                    isr.wy = false
                }
            }
        }
    }
    // temp

    // isr.0 = false;
}
