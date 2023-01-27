use bevy::prelude::*;
// use bevy_editor_pls::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_infinite_grid::{InfiniteGrid, InfiniteGridBundle, InfiniteGridPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_obj::*;

use crossbeam_channel::{unbounded, Receiver, Sender};

use anyhow::Context as _;
use std::io::Read;
use std::io::Write as _;
use std::path::Path;

fn main() {
    App::new()
        .insert_resource(Port(open(Path::new("/dev/ttyUSB0"), 115200)))
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
        .add_system(rotate)
        .add_plugin(WorldInspectorPlugin)
        .run();
}

#[derive(Component)]
struct AngleBef(f32);

#[derive(Component)]
struct AngleCur(f32);

#[derive(Component)]
struct AngleAft(f32);

#[derive(Component)]
struct AngleCalib(f32);

#[derive(Component)]
struct RotationBef(Quat);

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
    ass: Res<AssetServer>,
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
            AngleBef(0.0),
            AngleCur(0.0),
            AngleAft(0.0),
            AngleCalib(0.0),
            Rotor::Sy,
            RotationBef(Quat::IDENTITY),
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
            AngleBef(0.0),
            AngleCur(0.0),
            AngleAft(0.0),
            AngleCalib(0.0),
            Rotor::Sz,
            RotationBef(Quat::IDENTITY),
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
            AngleBef(0.0),
            AngleCur(0.0),
            AngleAft(0.0),
            AngleCalib(0.0),
            Rotor::Ez,
            RotationBef(Quat::IDENTITY),
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
            AngleBef(0.0),
            AngleCur(0.0),
            AngleAft(0.0),
            AngleCalib(0.0),
            Rotor::Wz,
            RotationBef(Quat::IDENTITY),
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
            AngleBef(0.0),
            AngleCur(0.0),
            AngleAft(0.0),
            AngleCalib(0.0),
            Rotor::Wy,
            RotationBef(Quat::IDENTITY),
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
    port: ResMut<Port>,
    isr: ResMut<IsRunning>,
    mut query: Query<(
        &mut Transform,
        &mut AngleBef,
        &mut AngleAft,
        &mut AngleCur,
        &mut AngleCalib,
        &Rotor,
        &mut RotationBef,
    )>,
) {
    egui::Window::new("Arm Control").show(egui_context.ctx_mut(), |ui| {
        let mut v: Vec<_> = query.iter_mut().collect();
        v.sort_by(|a, b| a.5.cmp(b.5));
        for (_, _, mut a, _, ac, r, _) in v {
            match r {
                Rotor::Sy => {
                    ui.add(egui::Slider::new(&mut a.0, -ac.0..=ac.0));
                }
                Rotor::Sz => {
                    ui.add(egui::Slider::new(&mut a.0, -ac.0..=ac.0));
                }
                Rotor::Ez => {
                    ui.add(egui::Slider::new(&mut a.0, -ac.0..=ac.0));
                }
                Rotor::Wz => {
                    ui.add(egui::Slider::new(&mut a.0, -ac.0..=ac.0));
                }
                Rotor::Wy => {
                    // ui.add(egui::Slider::new(&mut a.0, -ac.0..=ac.0));
                }
            }
        }
        if ui.button("run").clicked() {
            run_simulation(isr, port, query);
        } else if ui.button("Calibrate").clicked() {
            calibrate(port, query);
        }
    });
}

fn calibrate(
    mut port: ResMut<Port>,
    mut query: Query<(
        &mut Transform,
        &mut AngleBef,
        &mut AngleAft,
        &mut AngleCur,
        &mut AngleCalib,
        &Rotor,
        &mut RotationBef,
    )>,
) {
    if let Some((write_to_port, read_from_port)) = port.0.as_mut() {
        println!("sending calib com");
        write_to_port.send("1 0 0 0 0".to_string()).unwrap();
        // let mut buf = [0u8; 4098];
        // let mut bport = BufReader::new(port);
        // let mut buf = String::new();
        // let mut did_read = false;
        println!("start reading");
        let buf = read_from_port.recv().unwrap().trim().to_string();
        let v: Vec<&str> = buf.split(' ').collect();
        let sy = v[0].to_string();
        let sz = v[1].to_string();
        let ez = v[2].to_string();
        let wz = v[3].to_string();
        println!("{:#?}", buf);
        for (_, _, _, _, mut ac, r, _) in query.iter_mut() {
            match r {
                Rotor::Sy => {
                    println!("read {buf}");
                    ac.0 = (sy.parse::<i32>().unwrap() / 960) as f32;
                }
                Rotor::Sz => {
                    println!("read {buf}");
                    ac.0 = (sz.parse::<i32>().unwrap() / 960) as f32;
                }
                Rotor::Ez => {
                    println!("read {buf}");
                    ac.0 = (ez.parse::<i32>().unwrap() / 680) as f32;
                }
                Rotor::Wz => {
                    println!("read {buf}");
                    ac.0 = (wz.parse::<i32>().unwrap() / 555) as f32;
                }
                Rotor::Wy => (),
            }
        }
    }
}

fn run_simulation(
    mut isr: ResMut<IsRunning>,
    mut port: ResMut<Port>,
    mut query: Query<(
        &mut Transform,
        &mut AngleBef,
        &mut AngleAft,
        &mut AngleCur,
        &mut AngleCalib,
        &Rotor,
        &mut RotationBef,
    )>,
) {
    debug!("Simulation started!");
    isr.sy = true;
    isr.sz = true;
    isr.ez = true;
    isr.wz = true;
    isr.wy = true;

    let (mut sy, mut sz, mut ez, mut wz, mut wy) = (0_i64, 0_i64, 0_i64, 0_i64, 0_i64);

    for (mut t, mut b, a, mut c, _ac, r, rotb) in query.iter_mut() {
        t.rotation = rotb.0;
        // TODO исправить множители
        match r {
            Rotor::Sy => sy = (a.0 - b.0) as i64 * 960,
            Rotor::Sz => sz = (a.0 - b.0) as i64 * 960,
            Rotor::Ez => ez = (a.0 - b.0) as i64 * 680,
            Rotor::Wz => wz = (a.0 - b.0) as i64 * 555,
            Rotor::Wy => wy = (a.0 - b.0) as i64 * 555,
        }

        c.0 = b.0;
        b.0 = a.0;
    }
    if let Some((tx, _)) = &mut port.0.as_mut() {
        tx.send(format!("0 {sy} {sz} {ez} {wz}")).unwrap();
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
        self.sy || self.sz || self.ez || self.wz || self.wy
    }
}

// fn is_running(isr: Res<IsRunning>) -> bool {
//     isr.is_running()
// }

fn rotate(
    time: Res<Time>,
    mut isr: ResMut<IsRunning>,
    mut query: Query<(
        &mut Transform,
        &mut AngleBef,
        &mut AngleCur,
        &mut AngleAft,
        &Rotor,
        &mut RotationBef,
    )>,
) {
    for (mut t, _, mut c, a, r, mut rotb) in query.iter_mut() {
        match r {
            Rotor::Sy => {
                t.rotate_y(get_virtual_angle(a.0, c.0, time.delta_seconds()).to_radians());
                // println!("{:?} {:?}", r, t.rotation.to_axis_angle());
                // b.0 = (b.0 + (a.0 - b.0).signum());
                c.0 += get_virtual_angle(a.0, c.0, time.delta_seconds());
                // rotb.0 = t.rotation;
                if (c.0 - a.0).abs() < 0.31 {
                    isr.sy = false
                }
                if isr.is_running() {
                    rotb.0 = t.rotation;
                }
            }
            Rotor::Sz => {
                t.rotate_z(get_virtual_angle(a.0, c.0, time.delta_seconds()).to_radians());
                // println!("{:?} {:?}", r, t.rotation.to_axis_angle());
                // b.0 += (a.0 - b.0).signum();
                c.0 += get_virtual_angle(a.0, c.0, time.delta_seconds());
                // rotb.0 = t.rotation;
                if (c.0 - a.0).abs() < 0.31 {
                    isr.sz = false
                }
                if isr.is_running() {
                    rotb.0 = t.rotation;
                }
            }
            Rotor::Ez => {
                t.rotate_z(get_virtual_angle(a.0, c.0, time.delta_seconds()).to_radians());
                // println!("{:?} {:?}", r, t.rotation.to_axis_angle());
                // b.0 += (a.0 - b.0).signum();
                c.0 += get_virtual_angle(a.0, c.0, time.delta_seconds());
                // rotb.0 = t.rotation;
                if (c.0 - a.0).abs() < 0.31 {
                    isr.ez = false
                }
                if isr.is_running() {
                    rotb.0 = t.rotation;
                }
            }
            Rotor::Wz => {
                t.rotate_z(get_virtual_angle(a.0, c.0, time.delta_seconds()).to_radians());
                // println!("{:?} {:?}", r, t.rotation.to_axis_angle());
                // b.0 = (b.0 + (a.0 - b.0).signum());
                c.0 += get_virtual_angle(a.0, c.0, time.delta_seconds());
                // rotb.0 = t.rotation.clone();
                if (c.0 - a.0).abs() < 0.31 {
                    isr.wz = false
                }
                if isr.is_running() {
                    rotb.0 = t.rotation;
                }
            }
            Rotor::Wy => {
                t.rotate_y(get_virtual_angle(a.0, c.0, time.delta_seconds()).to_radians());
                // println!("{:?} {:?}", r, t.rotation.to_axis_angle());
                // b.0 += (a.0 - b.0).signum();
                c.0 += get_virtual_angle(a.0, c.0, time.delta_seconds());
                // rotb.0 = t.rotation.clone();
                if (c.0 - a.0).abs() < 0.31 {
                    isr.wy = false
                }
                if isr.is_running() {
                    rotb.0 = t.rotation;
                }
            }
        }
    }
    // temp

    // isr.0 = false;
}

#[inline]
fn get_virtual_angle(after: f32, before: f32, delta_secs: f32) -> f32 {
    // println!("{delta_secs}");
    if (after - before) > 0.3 {
        ((after - before) * delta_secs).clamp(0.3, 1.)
    } else if (after - before) < -0.3 {
        ((after - before) * delta_secs).clamp(-1., -0.3)
    } else {
        0.
    }
}

#[derive(Default, Resource)]
struct Port(Option<(Sender<String>, Receiver<String>)>);

pub fn open(port: &std::path::Path, baudrate: u32) -> Option<(Sender<String>, Receiver<String>)> {
    let rx = serialport::new(port.to_string_lossy(), baudrate)
        .timeout(std::time::Duration::from_secs(2))
        .open_native()
        .with_context(|| format!("failed to open serial port `{}`", port.display()))
        .ok();

    if let Some(mut port) = rx {
        // Set a CTRL+C handler to terminate cleanly instead of with an error.
        ctrlc::set_handler(move || {
            eprintln!();
            eprintln!("Exiting.");
            std::process::exit(0);
        })
        .context("failed setting a CTRL+C handler")
        .unwrap();

        let (write_to_port, inrx) = unbounded::<String>();
        let (outtx, read_from_port) = unbounded::<String>();

        std::thread::spawn(move || -> ! {
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
                if let Ok(ins) = inrx.try_recv() {
                    port.write_all(ins.as_bytes()).unwrap();
                    port.flush().unwrap();
                }

                let mut buf = [0u8; 4098];
                let trim_f = &['\r', '\n', '\0'];

                match port.read(&mut buf) {
                    Ok(_count) => {
                        outtx
                            .send(
                                String::from_utf8_lossy(&buf)
                                    .trim_end_matches(trim_f)
                                    .to_string(),
                            )
                            .unwrap();
                    }
                    Err(e) => {
                        assert!(e.kind() == std::io::ErrorKind::TimedOut);
                    }
                }
            }
        });

        // Some(port)
        Some((write_to_port, read_from_port))
    } else {
        None
    }

    // loop {
    //     let mut buf = [0u8; 4098];
    //     let count = stdin.read(&mut buf)?;
    //     tx.write_all(&buf[..count])?;
    //     tx.flush()?;
    // }
}
