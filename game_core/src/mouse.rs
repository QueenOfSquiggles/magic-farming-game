use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MouseState>();
        app.add_systems(OnEnter(MouseState::Free), state_mouse_free);
        app.add_systems(OnEnter(MouseState::Locked), state_mouse_lock);
        app.observe(watch_for_state_requests);
        if cfg!(debug_assertions) {
            app.add_systems(Update, dev_toggle_mouse_keypress);
        }
    }
}

#[derive(Event, Clone, Debug, Hash, PartialEq, Eq)]
pub struct MouseStateRequest(MouseState);

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum MouseState {
    #[default]
    Locked,
    Free,
}

fn dev_toggle_mouse_keypress(
    keys: Res<ButtonInput<KeyCode>>, current: Res<State<MouseState>>,
    mut next: ResMut<NextState<MouseState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next.set(match *current.get() {
            MouseState::Locked => MouseState::Free,
            MouseState::Free => MouseState::Locked,
        });
    }
}

fn watch_for_state_requests(
    trigger: Trigger<MouseStateRequest>, mut next: ResMut<NextState<MouseState>>,
) {
    next.set(trigger.event().0.clone());
}

fn state_mouse_lock(mut query: Query<&mut Window, With<PrimaryWindow>>) {
    let Ok(mut window) = query.get_single_mut() else {
        return;
    };
    window.cursor.grab_mode = CursorGrabMode::Locked;
    window.cursor.visible = false;
}

fn state_mouse_free(mut query: Query<&mut Window, With<PrimaryWindow>>) {
    let Ok(mut window) = query.get_single_mut() else {
        return;
    };
    window.cursor.grab_mode = CursorGrabMode::None;
    window.cursor.visible = true;
}
