use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, send_bevy_events);
}

/// Event occurring when two colliders start colliding
/// This will only get triggered if the entity has the
/// [`ActiveEvents::COLLISION_EVENTS`] flag enabled.
#[derive(Event, Copy, Clone, Debug, PartialEq, Eq)]
pub struct OnCollisionStart {
    pub collider: Entity,
    pub flags: CollisionEventFlags,
}

/// Event occurring when two colliders stop colliding
/// This will only get triggered if the entity has the
/// [`ActiveEvents::COLLISION_EVENTS`] flag enabled.
#[derive(Event, Copy, Clone, Debug, PartialEq, Eq)]
pub struct OnCollisionStop {
    pub collider: Entity,
    pub flags: CollisionEventFlags,
}

/// Generates bevy events for any physics interactions that have happened
/// that are stored in the events list
pub fn send_bevy_events(
    mut commands: Commands,
    mut collision_event_reader: EventReader<CollisionEvent>,
    //contact_force_event_writer: &mut EventReader<ContactForceEvent>,
) {
    // TODO: ? integrate it into rapier built-in `send_bevy_events` ?
    for collision_event in collision_event_reader.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, flags) => {
                commands.trigger_targets(
                    OnCollisionStart {
                        collider: *entity2,
                        flags: *flags,
                    },
                    *entity1,
                );
                commands.trigger_targets(
                    OnCollisionStart {
                        collider: *entity1,
                        flags: *flags,
                    },
                    *entity2,
                );
            }
            CollisionEvent::Stopped(entity1, entity2, flags) => {
                commands.trigger_targets(
                    OnCollisionStop {
                        collider: *entity2,
                        flags: *flags,
                    },
                    *entity1,
                );
                commands.trigger_targets(
                    OnCollisionStop {
                        collider: *entity1,
                        flags: *flags,
                    },
                    *entity2,
                );
            }
        }
    }
}
