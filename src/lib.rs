use bevy::prelude::*;

pub mod animated_interaction;
pub mod flux_interaction;
pub mod interactive_background;
pub mod interactive_border;
pub use flux_interaction::*;
use interactive_border::InteractiveBorderPlugin;

use self::{
    animated_interaction::AnimatedInteractionPlugin,
    interactive_background::InteractiveBackgroundPlugin,
};

pub struct SickleUiPlugin;

impl Plugin for SickleUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FluxInteractionPlugin,
            AnimatedInteractionPlugin,
            InteractiveBackgroundPlugin,
            InteractiveBorderPlugin,
        ));
    }
}

#[derive(Component)]
pub struct PointerTracker {
    pub pointer_over: bool,
    pub enter_timer: Timer,
    pub exit_timer: Timer,
    pub pointer_delta: Vec2,
}
