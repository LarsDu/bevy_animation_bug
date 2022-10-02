use bevy::prelude::*;
use bevy::{input::keyboard::*, prelude::*};
use std::f32::consts::PI;

pub struct Animations(pub Vec<Handle<AnimationClip>>);

pub const STAND_FORWARD: usize = 5;
pub const STAND_UP: usize = 1;
pub const STAND_ANGLED: usize = 0;
pub const WALK_FORWARD: usize = 2;
pub const WALK_UP: usize = 4;
pub const WALK_ANGLED: usize = 3;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HeroPlugin)
        .run();
}

#[derive(Component)]
pub struct Hero;

#[derive(Component)]
pub struct Movement {
    pub look: Vec2,
}

pub struct HeroPlugin;

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(init_animation)
            //.add_system(keyboard_directly_controls_animation); // <--- This works
            .add_system(keyboard_controls_look)// <--- But these do not!
            .add_system(set_animation_direction.after(keyboard_controls_look));// <--But these do not!
    }
}

fn init_animation(
    animations: Res<Animations>,
    mut query: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
) {
    if !*done {
        if let Ok(mut animation_player) = query.get_single_mut() {
            animation_player.play(animations.0[0].clone_weak()).repeat();
            *done = true;
        }
    }
}

fn keyboard_directly_controls_animation(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Movement, &mut Transform), With<Hero>>,
    animations: Res<Animations>,
    mut animation_player: Query<&mut AnimationPlayer>,
) {
    if let Ok(mut animation_player) = animation_player.get_single_mut() {
        // Insert and remove components based on keyboard input
        let (mut movement, mut transform) =
            query.get_single_mut().expect("Error! Couldn't find hero");

        if keyboard_input.just_pressed(KeyCode::Left) {
            transform.look_at(Vec3::new(-100.0, 0.0, 0.0), Vec3::Y);
            animation_player
                .play(animations.0[WALK_FORWARD].clone_weak())
                .repeat();
            movement.look.x = -1.0;
        } else if keyboard_input.just_released(KeyCode::Left) {
            animation_player
                .play(animations.0[STAND_FORWARD].clone_weak())
                .repeat();
            movement.look.x = 0.0;
        }

        if keyboard_input.just_pressed(KeyCode::Right) {
            transform.look_at(Vec3::new(100.0, 0.0, 0.0), Vec3::Y);
            movement.look.x = 1.0;
            animation_player
            .play(animations.0[WALK_FORWARD].clone_weak())
            .repeat();
        } else if keyboard_input.just_released(KeyCode::Right) {
            animation_player
                .play(animations.0[STAND_FORWARD].clone_weak())
                .repeat();
            movement.look.x = 0.0;
        }

        // UP
        if keyboard_input.just_pressed(KeyCode::Up) {
            animation_player
                .play(animations.0[STAND_UP].clone_weak())
                .repeat();
        } else if keyboard_input.just_released(KeyCode::Up) {
            animation_player
                .play(animations.0[STAND_FORWARD].clone_weak())
                .repeat();
        }
    }
}

fn keyboard_controls_look(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Movement, With<Hero>>,
) {
    let mut movement = query.get_single_mut().expect("Error! Couldn't find hero");

    if keyboard_input.just_pressed(KeyCode::Left) {
        movement.look.x = -1.0;
    } else if keyboard_input.just_released(KeyCode::Left) {
        movement.look.x = 0.0;
    }

    if keyboard_input.just_pressed(KeyCode::Right) {
        movement.look.x = 1.0;
    } else if keyboard_input.just_released(KeyCode::Right) {
        movement.look.x = 0.0;
    }
}

fn set_animation_direction(
    animations: Res<Animations>,
    mut query: Query<(&mut Movement, &mut Transform), With<Hero>>,
    mut animation_player: Query<&mut AnimationPlayer>,
) {
    if let Ok(mut animation_player) = animation_player.get_single_mut() {
        for (movement, mut transform) in &mut query {
            if movement.look.x.abs() > 0.0 {
                animation_player
                    .play(animations.0[WALK_FORWARD].clone_weak())
                    .repeat();
            } else {
                animation_player
                    .play(animations.0[STAND_FORWARD].clone_weak())
                    .repeat();
            }
            if movement.look.x > 0.0 {
                transform.look_at(Vec3::new(100.0, 0.0, 0.0), Vec3::Y);
            } else if movement.look.x < 0.0 {
                transform.look_at(Vec3::new(-100.0, 0.0, 0.0), Vec3::Y);
            }
        }
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 160.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Directional Light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 15000.0,
            color: Color::WHITE,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    // Insert animations
    commands.insert_resource(Animations(vec![
        asset_server.load("models/animated/dojoman.glb#Animation0"),
        asset_server.load("models/animated/dojoman.glb#Animation1"),
        asset_server.load("models/animated/dojoman.glb#Animation2"),
        asset_server.load("models/animated/dojoman.glb#Animation3"),
        asset_server.load("models/animated/dojoman.glb#Animation4"),
        asset_server.load("models/animated/dojoman.glb#Animation5"),
    ]));

    // Load the model
    let dojoman = asset_server.load("models/animated/dojoman.glb#Scene0");
    commands
        .spawn_bundle(SceneBundle {
            scene: dojoman,
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0.0,
                -PI * 0.5,
                0.0,
            )), //.with_scale(Vec3::splat(0.60)),
            ..default()
        })
        .insert(Hero)
        .insert(Movement {
            look: Vec2::new(0.0, 0.0),
        });
}
