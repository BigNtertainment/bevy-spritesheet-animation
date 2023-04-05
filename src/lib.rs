use animation::AnimationPlugin;
use bevy::prelude::*;

pub mod animation;

pub struct SpritesheetAnimationPlugin;

impl Plugin for SpritesheetAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AnimationPlugin);
    }
}