#![allow(dead_code)]
// this is gonna be full of "dead code" all the time since I do some degree of
// data planning with events

use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct PlayerInteractionChanged(pub Option<Entity>);

#[derive(Event, Debug, Clone)]
pub struct CropStageChange {
    pub entity: Entity,
    pub position: Vec3,
    pub name: Option<Name>,
}

#[derive(Event, Debug, Clone)]
pub struct PlayerInteract;

pub struct CommonEventsPlugin;

#[derive(Event, Debug, Clone)]
pub struct NewDay;

impl Plugin for CommonEventsPlugin {
    fn build(&self, app: &mut App) {
        app.observe(easy_event_print::<PlayerInteractionChanged>);
        app.observe(easy_event_print::<CropStageChange>);
        app.observe(easy_event_print::<PlayerInteract>);
        app.observe(easy_event_print::<NewDay>);
    }
}

/// Internal func to easily set up event printing
fn easy_event_print<E: std::fmt::Debug>(trigger: Trigger<E>) {
    debug!("EVENT ({}): {:?} ", trigger.entity(), trigger.event());
}
