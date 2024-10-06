use avian3d::prelude::{
    Collider, ColliderConstructor, ColliderConstructorHierarchy, CollisionLayers, LayerMask,
    MassPropertiesBundle, RigidBody,
};
use bevy::prelude::*;

use crate::{
    collision::GameLayers, common_events::PlayerInteract, data::game_asset_path::GameAssetPath,
};

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_interactables);
    }
}

#[derive(Component)]
pub struct Interactable;

fn create_interactables(mut cmd: Commands, assets: Res<AssetServer>) {
    cmd.spawn((
        Name::new("Test Interactable Object"),
        RigidBody::Dynamic,
        Interactable,
        CollisionLayers::new(
            [GameLayers::Default, GameLayers::Interactable],
            LayerMask::ALL,
        ),
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexDecompositionFromMesh),
        SceneBundle {
            scene: assets.load(GameAssetPath::new_model("::crate-color.glb").gltf_scene()),
            transform: Transform::from_xyz(2., 3., 0.).with_scale(Vec3::ONE * 5.),
            ..default()
        },
        MassPropertiesBundle::new_computed(&Collider::cuboid(5., 5., 5.), 5.0),
    ))
    .observe(handle_interact);
}

fn handle_interact(event: Trigger<PlayerInteract>) {
    info!("Received event at interactable root {}", event.entity());
}
