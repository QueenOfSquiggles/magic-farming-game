use std::time::Duration;

use bevy::prelude::*;
use bevy_hanabi::{EffectAsset, ParticleEffect, ParticleEffectBundle};
use crop_vfx::CropVfx;

use crate::data::named_asset_id::NamedAssets;

pub struct VfxPlugin;
pub mod crop_vfx;

impl Plugin for VfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CropVfx);
        app.init_asset::<VfxAsset>();
        app.init_resource::<NamedAssets<VfxAsset>>();
        app.add_systems(Update, despawn_vfx);
        app.observe(spawn_vfx);
    }
}

#[derive(Event, Debug)]
struct SpawnVfx {
    id: String,
    transform: Transform,
}

#[derive(Component)]
pub struct VfxOneshot(Timer);

#[derive(Asset, Reflect, Clone, Debug, Default)]
pub struct VfxAsset {
    id: String,
    effect: Handle<EffectAsset>,
    oneshot_duration: Option<Duration>,
}

impl VfxAsset {
    pub fn from_asset(
        name: impl Into<String>, effect: Handle<EffectAsset>,
        oneshot_duration: impl Into<Option<Duration>>,
    ) -> Self {
        Self {
            id: name.into(),
            effect,
            oneshot_duration: oneshot_duration.into(),
        }
    }
}

fn despawn_vfx(mut query: Query<(&mut VfxOneshot, Entity)>, time: Res<Time>, mut cmd: Commands) {
    for (mut effect, entity) in query.iter_mut() {
        effect.0.tick(time.delta());
        if effect.0.just_finished() {
            cmd.entity(entity).despawn();
        }
    }
}

fn spawn_vfx(
    trigger: Trigger<SpawnVfx>, mut cmd: Commands, names: Res<NamedAssets<VfxAsset>>,
    effects: Res<Assets<VfxAsset>>,
) {
    // info!("Received vfx spawn event : {:#?}", trigger.event());

    let SpawnVfx { id, transform } = trigger.event();
    let Some(vfx) = names.get(id) else {
        error!("Failed to find VFX data for {} (from available assets)", id);
        return;
    };
    let Some(vfx) = effects.get(vfx.id()) else {
        return;
    };
    info!("Spawning VFX: {} at {}", vfx.id, transform.translation());
    let mut e = cmd.spawn((
        Name::new(format!("VFX {} instance", vfx.id)),
        ParticleEffectBundle {
            effect: ParticleEffect::new(vfx.effect.clone()),
            transform: transform.clone(),
            ..default()
        },
    ));
    if let Some(dur) = vfx.oneshot_duration {
        e.insert(VfxOneshot(Timer::from_seconds(
            dur.as_secs_f32(),
            TimerMode::Once,
        )));
    }
}
