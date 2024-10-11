use std::{fs::File, time::Duration};

use bevy::prelude::*;
use bevy_hanabi::{
    AccelModifier, Attribute, ColorOverLifetimeModifier, EffectAsset, Gradient, LinearDragModifier,
    Module, OrientMode, OrientModifier, RoundModifier, SetAttributeModifier,
    SetPositionSphereModifier, SetVelocityTangentModifier, ShapeDimension,
    SizeOverLifetimeModifier, Spawner,
};
use ron::ser::PrettyConfig;

use crate::{
    common_events::CropStageChange,
    data::{game_asset_path::GameAssetPath, named_asset_id::NamedAssets},
};

use super::{SpawnVfx, VfxAsset};

pub struct CropVfx;

impl Plugin for CropVfx {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_vfx);
        app.observe(spawn_vfx);
    }
}

const VFX_CROP_STAGE_CHANGE: &'static str = "vfx_crop_stage_change";

fn init_vfx(
    mut cmd: Commands, mut names: ResMut<NamedAssets<VfxAsset>>,
    mut effects: ResMut<Assets<EffectAsset>>, mut containers: ResMut<Assets<VfxAsset>>,
) {
    let mut module = Module::default();
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(0.1),
        dimension: ShapeDimension::Volume,
    };

    let init_vel = SetVelocityTangentModifier {
        origin: module.lit(Vec3::ZERO),
        axis: module.lit(Vec3::Y),
        speed: module.lit(2.5),
    };
    let lifetime = SetAttributeModifier::new(Attribute::LIFETIME, module.lit(3.));

    let accel = AccelModifier::new(module.lit(Vec3::Y * 2.0));

    let mut gradient = Gradient::new();

    gradient.add_key(0.0, Color::srgb(0.0, 1.0, 0.0).to_linear().to_vec4());
    gradient.add_key(0.75, Color::srgb(0.0, 1.0, 0.0).to_linear().to_vec4());
    gradient.add_key(1.0, Color::srgb(1.0, 1.0, 1.0).to_linear().to_vec4());

    const PARTICLE_SIZE: f32 = 1.3;
    let mut size = Gradient::new();
    size.add_key(0.0, Vec2::ONE * 0.0 * PARTICLE_SIZE);
    size.add_key(0.1, Vec2::ONE * 1.0 * PARTICLE_SIZE);
    size.add_key(0.8, Vec2::ONE * 0.5 * PARTICLE_SIZE);
    size.add_key(1.0, Vec2::ONE * 0.0 * PARTICLE_SIZE);

    let round = RoundModifier {
        roundness: module.lit(1.),
    };

    let drag = LinearDragModifier::new(module.lit(2.));

    let effect = EffectAsset::new(vec![51], Spawner::once(50.0.into(), true), module)
        .with_name("Crop Stage Change VFX")
        .init(init_pos)
        .init(init_vel)
        .init(lifetime)
        .update(accel)
        .update(drag)
        .render(ColorOverLifetimeModifier { gradient })
        .render(OrientModifier {
            mode: OrientMode::FaceCameraPosition,
            rotation: None,
        })
        .render(SizeOverLifetimeModifier {
            gradient: size,
            screen_space_size: false,
        })
        .render(round);

    names.register(
        VFX_CROP_STAGE_CHANGE,
        containers.add(VfxAsset::from_asset(
            VFX_CROP_STAGE_CHANGE,
            effects.add(effect),
            Duration::from_secs_f32(3.5),
        )),
    );
}

fn spawn_vfx(trigger: Trigger<CropStageChange>, mut cmd: Commands) {
    // info!("Dispatching vfx spawn event");
    cmd.trigger(SpawnVfx {
        id: VFX_CROP_STAGE_CHANGE.into(),
        transform: Transform::from_translation(trigger.event().position),
    });
}
