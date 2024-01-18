use bevy::prelude::*;

pub mod animated_interaction;
pub mod drag_interaction;
pub mod flux_interaction;
pub mod interactions;
pub mod scroll_interaction;
pub mod widgets;
pub mod ui_builder;

use drag_interaction::DragInteractionPlugin;
pub use flux_interaction::*;
use interactions::InteractionsPlugin;
use scroll_interaction::ScrollInteractionPlugin;
use widgets::WidgetsPlugin;

use self::animated_interaction::AnimatedInteractionPlugin;

pub struct SickleUiPlugin;

impl Plugin for SickleUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AnimatedInteractionPlugin,
            DragInteractionPlugin,
            FluxInteractionPlugin,
            InteractionsPlugin,
            ScrollInteractionPlugin,
            WidgetsPlugin,
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
