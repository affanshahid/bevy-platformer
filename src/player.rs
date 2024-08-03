use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{animation::Animation, WINDOW_BOTTOM_Y, WINDOW_LEFT_X};

const PLAYER_VELOCITY_X: f32 = 400.0;
const PLAYER_VELOCITY_Y: f32 = 850.0;

const MAX_JUMP_HEIGHT: f32 = 230.0;

const SPRITESHEET_COLS: u32 = 7;
const SPRITESHEET_ROWS: u32 = 8;

const SPRITE_TILE_WIDTH: f32 = 128.0;
const SPRITE_TILE_HEIGHT: f32 = 256.0;

const SPRITE_RENDER_WIDTH: f32 = 64.0;
const SPRITE_RENDER_HEIGHT: f32 = 128.0;

const SPRITE_IDX_STAND: usize = 28;
const SPRITE_IDX_WALKING: &[usize] = &[7, 0];
const SPRITE_IDX_JUMP: usize = 35;

const CYCLE_DELAY: Duration = Duration::from_millis(70);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, movement)
            .add_systems(Update, jump)
            .add_systems(Update, rise)
            .add_systems(Update, fall)
            .add_systems(Update, apply_movement_animation)
            .add_systems(Update, apply_idle_sprite)
            .add_systems(Update, apply_jump_sprite)
            .add_systems(Update, update_direction)
            .add_systems(Update, update_sprite_direction);
    }
}

#[derive(Component)]
enum Direction {
    Right,
    Left,
}

fn setup(
    mut commands: Commands,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
    server: Res<AssetServer>,
) {
    let image_handle: Handle<Image> = server.load("spritesheets/spritesheet_players.png");
    let texture_atlas = TextureAtlasLayout::from_grid(
        UVec2::new(SPRITE_TILE_WIDTH as u32, SPRITE_TILE_HEIGHT as u32),
        SPRITESHEET_COLS,
        SPRITESHEET_ROWS,
        None,
        None,
    );
    let atlas_handle = atlases.add(texture_atlas);

    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite::default(),

                texture: image_handle,
                transform: Transform {
                    translation: Vec3::new(WINDOW_LEFT_X + 100.0, WINDOW_BOTTOM_Y + 300.0, 0.0),
                    scale: Vec3::new(
                        SPRITE_RENDER_WIDTH / SPRITE_TILE_WIDTH,
                        SPRITE_RENDER_HEIGHT / SPRITE_TILE_HEIGHT,
                        1.0,
                    ),
                    ..default()
                },
                ..default()
            },
            TextureAtlas {
                layout: atlas_handle,
                index: SPRITE_IDX_STAND,
            },
        ))
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(
            SPRITE_TILE_WIDTH / 2.0,
            SPRITE_TILE_HEIGHT / 2.0,
        ))
        .insert(KinematicCharacterController::default())
        .insert(Direction::Right);
}

fn movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut KinematicCharacterController>,
) {
    let mut player = query.single_mut();

    let mut movement = 0.0;

    if input.pressed(KeyCode::ArrowRight) {
        movement += time.delta_seconds() * PLAYER_VELOCITY_X;
    }

    if input.pressed(KeyCode::ArrowLeft) {
        movement += time.delta_seconds() * PLAYER_VELOCITY_X * -1.0;
    }

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(movement, vec.y)),
        None => player.translation = Some(Vec2::new(movement, 0.0)),
    }
}

#[derive(Component)]
struct Jump(f32);

#[allow(clippy::type_complexity)]
fn jump(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    query: Query<
        (Entity, &KinematicCharacterControllerOutput),
        (With<KinematicCharacterController>, Without<Jump>),
    >,
) {
    if query.is_empty() {
        return;
    }

    let (player, output) = query.single();

    if input.pressed(KeyCode::ArrowUp) && output.grounded {
        commands.entity(player).insert(Jump(0.0));
    }
}

fn rise(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut KinematicCharacterController, &mut Jump)>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut player, mut jump) = query.single_mut();

    let mut movement = time.delta().as_secs_f32() * PLAYER_VELOCITY_Y;

    if movement + jump.0 >= MAX_JUMP_HEIGHT {
        movement = MAX_JUMP_HEIGHT - jump.0;
        commands.entity(entity).remove::<Jump>();
    }

    jump.0 += movement;

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
        None => player.translation = Some(Vec2::new(0.0, movement)),
    }
}

fn fall(time: Res<Time>, mut query: Query<&mut KinematicCharacterController, Without<Jump>>) {
    if query.is_empty() {
        return;
    }

    let mut player = query.single_mut();
    let movement = time.delta().as_secs_f32() * (PLAYER_VELOCITY_Y / 1.5) * -1.0;

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
        None => player.translation = Some(Vec2::new(0.0, movement)),
    }
}

fn apply_movement_animation(
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput), Without<Animation>>,
) {
    if query.is_empty() {
        return;
    }

    let (player, output) = query.single();
    if output.desired_translation.x != 0.0 && output.grounded {
        commands
            .entity(player)
            .insert(Animation::new(SPRITE_IDX_WALKING, CYCLE_DELAY));
    }
}

fn apply_idle_sprite(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &KinematicCharacterControllerOutput,
        &mut TextureAtlas,
    )>,
) {
    if query.is_empty() {
        return;
    }

    let (player, output, mut sprite) = query.single_mut();
    if output.desired_translation.x == 0.0 && output.grounded {
        commands.entity(player).remove::<Animation>();
        sprite.index = SPRITE_IDX_STAND
    }
}

fn apply_jump_sprite(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &KinematicCharacterControllerOutput,
        &mut TextureAtlas,
    )>,
) {
    if query.is_empty() {
        return;
    }

    let (player, output, mut sprite) = query.single_mut();
    if !output.grounded {
        commands.entity(player).remove::<Animation>();
        sprite.index = SPRITE_IDX_JUMP
    }
}

fn update_direction(
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput)>,
) {
    if query.is_empty() {
        return;
    }

    let (player, output) = query.single();

    if output.desired_translation.x > 0.0 {
        commands.entity(player).insert(Direction::Right);
    } else if output.desired_translation.x < 0.0 {
        commands.entity(player).insert(Direction::Left);
    }
}

fn update_sprite_direction(mut query: Query<(&mut Sprite, &Direction)>) {
    if query.is_empty() {
        return;
    }

    let (mut sprite, direction) = query.single_mut();

    match direction {
        Direction::Right => sprite.flip_x = false,
        Direction::Left => sprite.flip_x = true,
    }
}
