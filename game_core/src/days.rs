use bevy::prelude::*;

use crate::common_events::NewDay;

pub struct DaysPlugin;

const DEBUG_SECONDS_PER_DAY: f32 = 45.0;

impl Plugin for DaysPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DayInfo {
            timer: Timer::from_seconds(DEBUG_SECONDS_PER_DAY, TimerMode::Repeating),
        });
        app.add_systems(Update, (inc_days_timed, debug_inc_days_keypress));
    }
}

#[derive(Resource, Clone, Debug)]
struct DayInfo {
    timer: Timer,
}

fn inc_days_timed(mut cmd: Commands, mut day: ResMut<DayInfo>, time: Res<Time>) {
    day.timer.tick(time.delta());
    if day.timer.just_finished() {
        cmd.trigger(NewDay);
    }
}
fn debug_inc_days_keypress(
    mut cmd: Commands, mut day: ResMut<DayInfo>, input: Res<ButtonInput<KeyCode>>,
) {
    if !input.just_pressed(KeyCode::NumpadEnter) {
        return;
    }
    day.timer.reset();
    cmd.trigger(NewDay);
}
