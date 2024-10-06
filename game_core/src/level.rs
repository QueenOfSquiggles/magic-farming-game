use avian3d::prelude::{Collider, CollisionLayers, LayerMask, RigidBody};
use bevy::{
    math::Affine2,
    prelude::*,
    render::texture::{
        ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor,
    },
};

use crate::{collision::GameLayers, data::game_asset_path::GameAssetPath};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, create_level);
    }
}

fn create_level(
    mut cmd: Commands, assets: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let texture = assets.load_with_settings::<Image, _>(
        GameAssetPath::new_texture("::Debug/Dark/texture_07.png"),
        |sampler: &mut ImageLoaderSettings| {
            // holy hell that's a lotta effort for repeating textures!
            *sampler = ImageLoaderSettings {
                sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::Repeat,
                    ..default()
                }),
                ..default()
            }
        },
    );
    cmd.spawn((
        Name::from("GroundPlane"),
        RigidBody::Static,
        Collider::half_space(Vec3::Y),
        CollisionLayers::new(GameLayers::Default, LayerMask::ALL),
        PbrBundle {
            // we have to use half-size for the UV?
            mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::new(25., 25.))),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture),
                uv_transform: Affine2::from_scale(Vec2::ONE * 50.),
                ..default()
            }),
            ..default()
        },
    ));
    cmd.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1000.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.5, 1., 0.5).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
