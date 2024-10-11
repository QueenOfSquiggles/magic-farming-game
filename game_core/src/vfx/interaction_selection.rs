use bevy::prelude::*;
use bevy_sprite3d::{Sprite3d, Sprite3dParams, Sprite3dPlugin};

use crate::{common_events::PlayerInteractionChanged, data::game_asset_path::GameAssetPath};
pub struct InteractionSelectionPlugin;

impl Plugin for InteractionSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Sprite3dPlugin);
        app.add_systems(Startup, spawn_cursor);
        app.observe(update_cursor);
    }
}

#[derive(Component)]
struct Cursor;

fn spawn_cursor(mut cmd: Commands, assets: Res<AssetServer>, mut sprite_params: Sprite3dParams) {
    cmd.spawn((
        Name::new("Selection Cursor"),
        Cursor,
        Sprite3d {
            image: assets.load(GameAssetPath::new_texture("::Debug/Dark/texture_06.png")),
            pixels_per_metre: 1024.,
            unlit: true,
            alpha_mode: AlphaMode::Mask(0.5),
            ..default()
        }
        .bundle(&mut sprite_params),
        Visibility::Hidden,
    ));
}
fn update_cursor(
    trigger: Trigger<PlayerInteractionChanged>,
    mut query: Query<(&mut Transform, Entity), With<Cursor>>,
    get_transform_query: Query<&GlobalTransform, Without<Cursor>>, mut cmd: Commands,
) {
    let Ok((mut trans, e)) = query.get_single_mut() else {
        return;
    };
    match trigger.event().0 {
        Some(target) => {
            cmd.entity(e).insert(Visibility::Visible);
            let Ok(target_trans) = get_transform_query.get(target) else {
                return;
            };
            trans.translation = target_trans.translation();
        }
        None => {
            cmd.entity(e).insert(Visibility::Hidden);
        }
    }
}
