use std::{cell::Ref, f32::consts::PI};

use bevy::{
    input::gamepad::{self, AxisSettings, ButtonSettings, GamepadInfo, GamepadSettings},
    prelude::*,
};

mod bullet;
pub mod physics;
pub mod target;
pub mod tower;
pub mod common;

use bevy_rapier3d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use bullet::*;
use target::*;
use tower::*;
use common::*;

const HEIGHT: f32 = 720.0;
const WIDTH: f32 = 1280.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_startup_system_to_stage(StartupStage::PreStartup, asset_loading)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_basic_scene)
        .register_type::<Tower>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: "Demo".to_string(),
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
        .add_plugin(target::TargetPlugin)
        .add_plugin(bullet::BulletPlugin)
        .add_plugin(tower::TowerPlugin)
        .add_plugin(physics::PhysicsPlugin)
        .add_system(gamepad_connections)
        .add_system(camera_controls)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<GameAssets>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Name::new("Ground"));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Tower {
            shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            bullet_offset: Vec3::new(0.0, 0.2, 0.5),
        })
        .insert(Name::new("Tower"));

    let mut target_ts = Transform::from_xyz(-2.0, 0.0, 1.5);
    target_ts.rotate_local_y(45.0);

    // commands
    //     .spawn(TargetBundle::from(SceneBundle {
    //         scene: assets.target_scene.clone(),
    //         transform: target_ts,
    //         ..default()
    //     }));
    //
    // let mut target_ts = Transform::from_xyz(-4.0, 0.0, 1.5);
    // target_ts.rotate_local_y(45.0);
    // commands
    //     .spawn(SceneBundle {
    //         scene: assets.target_scene.clone(),
    //         transform: target_ts,
    //         ..default()
    //     })
    //     .insert(Target { speed: 0.3 })
    //     .insert(Health { value: 3 })
    //     .insert(Name::new("Target"));

    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("Light"));
}



fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bullet_scene: assets.load("Bullet1.glb#Scene0"),
        target_scene: assets.load("Target.glb#Scene0"),
    });
}

#[derive(Resource)]
pub struct GameAssets {
    bullet_scene: Handle<Scene>,
    target_scene: Handle<Scene>,
}

fn camera_controls(
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    keyboard: Res<Input<KeyCode>>,
    gamepad: Option<Res<MyGamepad>>,
    gamepad_buttons: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();
    let mut forward = camera.forward();
    let mut back = camera.back();
    let left = camera.left();
    let right = camera.right();

    forward.y = 0.0;
    forward = forward.normalize();

    let speed = 3.0;
    let rotete_speed = 1.5;

    if keyboard.pressed(KeyCode::W) {
        camera.translation += forward * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::S) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::A) {
        camera.translation += left * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::D) {
        camera.translation += right * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Y, rotete_speed * time.delta_seconds());
    }

    if keyboard.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Y, -rotete_speed * time.delta_seconds());
    }

    // gamepad

    if let Some(gamepad) = gamepad {
        let gamepad = gamepad.0;

        for ev in gamepad_evr.iter() {
            if ev.gamepad != gamepad {
                // event not from our gamepad
                continue;
            }
            use GamepadEventType::{AxisChanged, ButtonChanged};

            match ev.event_type {
                AxisChanged(GamepadAxisType::RightStickX, x) => {
                    // Right Stick moved (X)
                    if x > 0.0 {
                        info!("right stick:{}", x);
                    }
                }
                AxisChanged(GamepadAxisType::RightStickY, y) => {
                    if y > 0.0 {
                        info!("right stick:{}", y);
                    }
                    // Right Stick moved (Y)
                }
                ButtonChanged(GamepadButtonType::DPadDown, val) => {
                    // buttons are also reported as analog, so use a threshold
                    if val > 0.5 {
                        // button pressed
                    }
                }
                _ => {} // don't care about other inputs
            }
        }

        // let axis_lx = GamepadAxis {
        //     gamepad: gamepad,
        //     axis_type: GamepadAxisType::LeftStickX,
        // };
        // let axis_ly = GamepadAxis {
        //     gamepad: gamepad,
        //     axis_type: GamepadAxisType::LeftStickY,
        // };

        // if let (Some(left_stick_x), Some(left_stick_y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
        //     // combine X and Y into one vector
        //     // let left_stick_pos = Vec2::new(x, y);

        //     // // Example: check if the stick is pushed up
        //     // if left_stick_pos.length() > 0.9 && left_stick_pos.y > 0.5 {
        //     //     // do something
        //     // }

        //     if left_stick_x.abs() > 0.1 {
        //         if left_stick_x > 0.0 {
        //             camera.translation += right * time.delta_seconds() * speed;
        //         } else {
        //             camera.translation += left * time.delta_seconds() * speed;
        //         }
        //     }

        //     if left_stick_y.abs() > 0.1 {
        //         if left_stick_y > 0.0 {
        //             camera.translation += forward * time.delta_seconds() * speed;
        //         } else {
        //             camera.translation -= forward * time.delta_seconds() * speed;
        //         }
        //     }
        // }

        // let axis_rx = GamepadAxis {
        //     gamepad: gamepad,
        //     axis_type: GamepadAxisType::RightStickX,
        // };
        // let axis_ry = GamepadAxis {
        //     gamepad: gamepad,
        //     axis_type: GamepadAxisType::RightStickY,
        // };
        // if let (Some(right_stick_x), Some(right_stick_y)) = (axes.get(axis_rx), axes.get(axis_ry)) {
        //     // combine X and Y into one vector
        //     // let left_stick_pos = Vec2::new(x, y);

        //     // // Example: check if the stick is pushed up
        //     // if left_stick_pos.length() > 0.9 && left_stick_pos.y > 0.5 {
        //     //     // do something
        //     // }
        //     info!("right stick, {},{}", right_stick_x, right_stick_y);

        //     if right_stick_x.abs() > 0.1 {
        //         if right_stick_x > 0.0 {
        //             camera.rotate_axis(Vec3::Y, -rotete_speed * time.delta_seconds());
        //         } else {
        //             camera.rotate_axis(Vec3::Y, rotete_speed * time.delta_seconds());
        //         }
        //     }
        // }
    }
}

#[derive(Resource)]
pub struct MyGamepad(Gamepad);

impl MyGamepad {
    fn new(id: usize) -> Self {
        Self(Gamepad::new(id))
    }
}

fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for ev in gamepad_evr.iter() {
        // the ID of the gamepad
        let id = ev.gamepad;
        match &ev.event_type {
            GamepadEventType::Connected(GamepadInfo { name }) => {
                println!("New gamepad connected with ID: {:?}, Name: {:?}", id, name);

                // if we don't have any gamepad yet, use this one
                if my_gamepad.is_none() {
                    commands.insert_resource(MyGamepad(id));
                }
            }
            GamepadEventType::Disconnected => {
                println!("Lost gamepad connection with ID: {:?}", id);

                // if it's the one we previously associated with the player,
                // disassociate it:
                if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                    if *old_id == id {
                        commands.remove_resource::<MyGamepad>();
                    }
                }
            }
            // other events are irrelevant
            _ => {}
        }
    }
}

// fn configure_gamepads(my_gamepad: Option<Res<MyGamepad>>, mut settings: ResMut<GamepadSettings>) {
//     let gamepad = if let Some(gp) = my_gamepad {
//         // a gamepad is connected, we have the id
//         gp.0
//     } else {
//         // no gamepad is connected
//         return;
//     };

//     // add a larger default dead-zone to all axes (ignore small inputs, round to zero)
//     settings.default_axis_settings.negative_low = -0.1;
//     settings.default_axis_settings.positive_low = 0.1;

//     // make the right stick "binary", squash higher values to 1.0 and lower values to 0.0
//     let right_stick_settings = AxisSettings {
//         positive_high: 0.5,  // values  0.5 to  1.0, become  1.0
//         positive_low: 0.5,   // values  0.0 to  0.5, become  0.0
//         negative_low: -0.5,  // values -0.5 to  0.0, become  0.0
//         negative_high: -0.5, // values -1.0 to -0.5, become -1.0
//         // the raw value should change by at least this much,
//         // for Bevy to register an input event:
//         threshold: 0.01,
//     };

//     // make the triggers work in big/coarse steps, to get fewer events
//     // reduces noise and precision
//     let trigger_settings = AxisSettings {
//         threshold: 0.2,
//         // also set some conservative deadzones
//         positive_high: 0.8,
//         positive_low: 0.2,
//         negative_high: -0.8,
//         negative_low: -0.2,
//     };

//     // set these settings for the gamepad we use for our player
//     settings.axis_settings.insert(
//         GamepadAxis {
//             gamepad,
//             axis_type: GamepadAxisType::RightStickX,
//         },
//         right_stick_settings.clone(),
//     );
//     settings.axis_settings.insert(
//         GamepadAxis {
//             gamepad,
//             axis_type: GamepadAxisType::RightStickY,
//         },
//         right_stick_settings.clone(),
//     );
//     settings.axis_settings.insert(
//         GamepadAxis {
//             gamepad,
//             axis_type: GamepadAxisType::LeftZ,
//         },
//         trigger_settings.clone(),
//     );
//     settings.axis_settings.insert(
//         GamepadAxis {
//             gamepad,
//             axis_type: GamepadAxisType::RightZ,
//         },
//         trigger_settings.clone(),
//     );

//     // for buttons (or axes treated as buttons), make them less sensitive
//     let button_settings = ButtonSettings {
//         // require them to be pressed almost all the way, to count
//         press: 0.9,
//         // require them to be released almost all the way, to count
//         release: 0.1,
//     };

//     settings.default_button_settings = button_settings;
// }
