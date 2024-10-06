use std::env;

use avian3d::PhysicsPlugins;
use bevy::{
    input::common_conditions::input_toggle_active,
    log::{Level, LogPlugin},
    prelude::*,
    window::PresentMode,
};
use bevy_hanabi::HanabiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rand::{
    plugin::EntropyPlugin,
    prelude::{GlobalEntropy, WyRand},
};
use bevy_screen_diagnostics::{
    ScreenDiagnosticsPlugin, ScreenEntityDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin,
};
use bevy_tweening::TweeningPlugin;

use common_events::CommonEventsPlugin;
use crops::CropsPlugin;
use days::DaysPlugin;
use hud::HudPlugin;
use interaction::InteractionPlugin;
use items::ItemsPlugin;
use level::LevelPlugin;
use mouse::MousePlugin;
use player::PlayerPlugin;
use settings::GameSettingsPlugin;
use vfx::VfxPlugin;
pub type Random = GlobalEntropy<WyRand>;

pub mod collision;
pub mod common_events;
pub mod constants;
pub mod crops;
pub mod data;
pub mod days;
pub mod hud;
pub mod interaction;
pub mod items;
pub mod level;
pub mod mouse;
pub mod player;
pub mod settings;
pub mod vfx;

pub struct GamePlugins;

impl GamePlugins {
    pub fn run_app() {
        App::new().add_plugins(GamePlugins).run();
    }
}

impl Plugin for GamePlugins {
    fn build(app: &mut App) {
        let args = env::args().collect::<Vec<String>>();
        let mut app = App::new();
        app.add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    level: if args.contains(&"--debug".into()) {
                        // force debug mode when user specifies
                        Level::DEBUG
                    } else {
                        if cfg!(debug_assertions) {
                            // if unspecified but in dev environment, use info
                            Level::INFO
                        } else {
                            // unspecifed release build, use warn
                            Level::WARN
                        }
                    },
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Magical Farm-Ranch-Cook-ing Game".into(),
                        name: Some("squiggles.mfrcg".into()),
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }),
        );
        app.insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.08)));
        app.insert_resource(AmbientLight {
            color: Color::srgb(0.1, 0.1, 0.1),
            brightness: 100.0,
        });
        app.insert_resource(Msaa::Sample4);
        // Third Party
        app.add_plugins(PhysicsPlugins::default());
        app.add_plugins(TweeningPlugin);
        app.add_plugins(ScreenDiagnosticsPlugin::default());
        app.add_plugins(ScreenFrameDiagnosticsPlugin);
        app.add_plugins(ScreenEntityDiagnosticsPlugin);
        app.add_plugins(EntropyPlugin::<WyRand>::default());
        app.add_plugins(HanabiPlugin);
        app.add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F1)),
        );

        // Self
        app.add_plugins((
            GameSettingsPlugin,
            LevelPlugin,
            PlayerPlugin,
            MousePlugin,
            InteractionPlugin,
            CommonEventsPlugin,
            HudPlugin,
            ItemsPlugin,
            CropsPlugin,
            DaysPlugin,
            VfxPlugin,
        ));
        app.add_systems(PostStartup, add_fallback_camera);
        if cfg!(debug_assertions) {
            // only do this in development (when debug assertions are available)
            app.add_systems(Update, exit_on_f8);
        }

        // Run App
        app.run()
    }
}

fn add_fallback_camera(mut commands: Commands, query: Query<&Camera>) {
    if query.is_empty() {
        // fallback in case a different module doesn't add a camera
        commands.spawn(Camera3dBundle::default());
    }
}

/// Entirely because I still have Godot muscle memory, I press F8 anyway when I
/// wanna close the app. So this will make that actually work
fn exit_on_f8(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::F8) {
        info!("Closing on F8 key press");
        exit.send(AppExit::Success);
    }
}
