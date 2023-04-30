use bevy::{prelude::*, window::WindowResolution};

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;

const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;

const COLOR_BACKGROUND: Color = Color::rgb(0.13, 0.13, 0.23);
const COLOR_PLATFORM: Color = Color::rgb(0.29, 0.31, 0.41);
const COLOR_PLAYER: Color = Color::rgb(0.60, 0.55, 0.60);

fn main() {
    App::new()
        .insert_resource(ClearColor(COLOR_BACKGROUND))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Platformer".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: COLOR_PLATFORM,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(-100.0, WINDOW_BOTTOM_Y, 0.0),
            scale: Vec3::new(75.0, 400.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: COLOR_PLATFORM,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(100.0, WINDOW_BOTTOM_Y, 0.0),
            scale: Vec3::new(50.0, 700.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: COLOR_PLATFORM,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(350.0, WINDOW_BOTTOM_Y, 0.0),
            scale: Vec3::new(150.0, 500.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(Camera2dBundle::default());
}
