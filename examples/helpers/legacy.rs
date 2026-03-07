//! Legacy replacement for [`Interaction`] (similar but humbler)
//!
//! Please do not use in new examples
//! Examples dependent on this need revision
//!
// # TODO
// The following examples need to be migrated away from this legacy system:
// - `examples/games/game_menu.rs`
// - `examples/bar.rs`

use bevy::prelude::*;
use std::collections::HashMap;

/// Pointer observer and (upcoming) ui_widgets are the recommended alternatives.
/// # What's not working
/// Persisting press on click, enter, or over
/// # How to get there
/// Local focus(Local<Option<Entity>>) + more readers with bubbling
pub fn handle_button_interaction(
    mut leave_reader: MessageReader<Pointer<Leave>>,
    mut out_reader: MessageReader<Pointer<Out>>,
    mut over_reader: MessageReader<Pointer<Over>>,
    mut press_reader: MessageReader<Pointer<Press>>,
    mut release_reader: MessageReader<Pointer<Release>>,
    mut button_interaction_query: Query<(Entity, &mut LegacyInteraction)>,
    children: Query<&Children>,
) {
    let mut buttons: HashMap<Entity, LegacyInteraction> = HashMap::new();

    for out in out_reader.read() {
        buttons.insert(out.event_target(), LegacyInteraction::None);
    }

    for leave in leave_reader.read() {
        buttons.insert(leave.event_target(), LegacyInteraction::None);
    }

    for over in over_reader.read() {
        buttons.insert(over.event_target(), LegacyInteraction::Hovered);
    }

    for press in press_reader.read() {
        buttons.insert(press.event_target(), LegacyInteraction::Pressed);
    }

    for release in release_reader.read() {
        buttons.insert(release.event_target(), LegacyInteraction::Hovered);
    }

    // Propagate(bubble) non-trivial interaction(non None) to a button ancestor if exists
    for (event_target, new_interaction) in buttons.clone() {
        if new_interaction != LegacyInteraction::None {
            for (button_entity, _) in button_interaction_query.iter_mut() {
                for descendant in children.iter_descendants(button_entity) {
                    if event_target == descendant {
                        buttons.remove(&event_target);
                        buttons.insert(button_entity, new_interaction.clone());
                    }
                }
            }
        }
    }

    for (event_target, new_interaction) in buttons {
        for (button_entity, mut interaction) in button_interaction_query.iter_mut() {
            if event_target == button_entity {
                *interaction = new_interaction.clone();
            }
        }
    }
}

#[derive(Component, PartialEq, Clone, Debug, Default)]
#[require(Button)]
pub enum LegacyInteraction {
    Pressed,
    Hovered,
    #[default]
    None,
}
