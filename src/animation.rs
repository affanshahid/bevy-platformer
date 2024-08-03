use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
pub struct Animation {
    pub sprites: &'static [usize],
    pub timer: Timer,
}

impl Animation {
    pub fn new(sprites: &'static [usize], delay: Duration) -> Self {
        Self {
            sprites,
            timer: Timer::new(delay, TimerMode::Repeating),
        }
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate);
    }
}

fn animate(mut query: Query<(&mut TextureAtlas, &mut Animation)>, time: Res<Time>) {
    for (mut sprite, mut animation) in query.iter_mut() {
        if animation.timer.tick(time.delta()).just_finished() {
            let current_idx = animation
                .sprites
                .iter()
                .position(|s| *s == sprite.index)
                .unwrap_or(0); // default to 0 if the current sprite is not in the set

            let next_idx = (current_idx + animation.timer.times_finished_this_tick() as usize)
                % animation.sprites.len();

            sprite.index = animation.sprites[next_idx];
        }
    }
}
