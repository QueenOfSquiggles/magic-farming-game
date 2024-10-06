use bevy::prelude::*;
use game_core::GamePlugins;

fn main() {
    match App::new().add_plugins(GamePlugins).run() {
        AppExit::Success => println!("Application exiting successfully"),
        AppExit::Error(non_zero) => {
            eprintln!("Application exiting (managed) with exit code: {}", non_zero)
        }
    }
}
