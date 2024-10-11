use bevy::{asset::LoadState, prelude::*};
use bevy_sprite3d::{Sprite3d, Sprite3dParams, Sprite3dPlugin};

use crate::{
    common_events::PlayerInteractionChanged, data::game_asset_path::GameAssetPath,
    dev_assertions::assert_dev,
};
pub struct InteractionSelectionPlugin;

impl Plugin for InteractionSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Sprite3dPlugin);
        app.add_systems(Startup, load_image);
        app.add_systems(Update, (spawn_cursor, cursor_face_camera));
        app.observe(update_cursor);
        app.init_resource::<CursorImage>();
    }
}

#[derive(Component)]
struct Cursor(Option<Entity>);

#[derive(Resource, Default)]
struct CursorImage(Handle<Image>);

fn load_image(assets: Res<AssetServer>, mut cursor_image: ResMut<CursorImage>) {
    cursor_image.0 = assets.load(GameAssetPath::new_texture("::crosshair117.png"));
}

fn spawn_cursor(
    mut cmd: Commands, assets: Res<AssetServer>, mut sprite_params: Sprite3dParams,
    mut has_run: Local<bool>, image: Res<CursorImage>,
) {
    if *has_run {
        return;
    }
    if assets.load_state(image.0.id()) != LoadState::Loaded {
        return;
    }
    *has_run = true;
    cmd.spawn((
        Name::new("Selection Cursor"),
        Cursor(None),
        Sprite3d {
            image: image.0.clone(),
            pixels_per_metre: 64.,
            unlit: true,
            alpha_mode: AlphaMode::Mask(0.5),
            ..default()
        }
        .bundle(&mut sprite_params),
    ))
    .insert(Visibility::Hidden);
}
fn update_cursor(
    trigger: Trigger<PlayerInteractionChanged>,
    mut query: Query<(&mut Transform, &mut Cursor, Entity)>,
    get_transform_query: Query<&GlobalTransform, Without<Cursor>>, mut cmd: Commands,
) {
    let Ok((mut trans, mut cursor, e)) = query.get_single_mut() else {
        return;
    };
    cursor.0 = trigger.event().0;
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

fn cursor_face_camera(
    query_camera: Query<&GlobalTransform, With<Camera>>,
    mut query_cursor: Query<&mut Transform, (Without<Camera>, With<Cursor>)>,
) {
    let Ok(cam_trans) = query_camera.get_single() else {
        assert_dev("Failed to find camera for cursor facing camera system");
        return;
    };
    let Ok(mut cursor_trans) = query_cursor.get_single_mut() else {
        // because of the delayed spawning it's entirely possible for the cursor to be missing for a few cycles

        // assert_dev("Failed to find cursor");

        return;
    };
    cursor_trans.look_at(cam_trans.translation(), Vec3::Y);
}
