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
    let scene = assets.load(GameAssetPath::new_model("::crate-color.glb").gltf_scene());

    spawn_interactable_crate(&mut cmd, scene.clone(), Vec3::new(-2., 10., 0.), "Crate A");
    spawn_interactable_crate(&mut cmd, scene.clone(), Vec3::new(0., 10., 2.), "Crate B");
    spawn_interactable_crate(&mut cmd, scene.clone(), Vec3::new(-2., 10., 2.), "Crate C");
    spawn_interactable_crate(&mut cmd, scene.clone(), Vec3::new(-2., 10., 2.), "Crate D");
}

fn spawn_interactable_crate(cmd: &mut Commands, scene: Handle<Scene>, position: Vec3, name: &str) {
    cmd.spawn((
        Name::new(name.to_string()),
        RigidBody::Dynamic,
        Interactable,
        CollisionLayers::new(
            [GameLayers::Default, GameLayers::Interactable],
            LayerMask::ALL,
        ),
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh)
            .with_default_density(0.2),
        SceneBundle {
            scene,
            transform: Transform::from_translation(position).with_scale(Vec3::ONE * 2.),
            ..default()
        },
    ))
    .observe(handle_interact);
}

fn handle_interact(event: Trigger<PlayerInteract>) {
    info!("Received event at interactable root {}", event.entity());
}
