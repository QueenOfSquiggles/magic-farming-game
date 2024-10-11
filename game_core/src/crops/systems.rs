use std::fs::File;

use avian3d::prelude::{
    Collider, ColliderConstructor, ColliderConstructorHierarchy, ColliderDensity, RigidBody,
    VhacdParameters,
};
use bevy::{ecs::system::EntityCommands, prelude::*, scene::SceneInstance};
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use ron::{extensions::Extensions, ser::PrettyConfig};

use crate::{
    common_events::{CropStageChange, NewDay},
    data::{game_asset_path::GameAssetPath, range::Range},
    items::{drops::ItemDrop, ItemId},
};

use super::{
    components::*,
    data::{CropDefinition, CropStage},
};

pub(super) fn emit_data_file() {
    let path_ron = GameAssetPath::new_data("::crops/example.ron");
    let Ok(file_ron) = File::create(path_ron.path_relative()) else {
        warn!("Emitting example crop failed due to bad path: {}", path_ron);
        return;
    };
    let item = CropDefinition {
        id: "Example".into(),
        stages: vec![
            CropStage {
                model: "::crate-color.glb".into(),
                duration: Range { min: 1, max: 2 },
                begin_status: Some(CropStatus::Growing),
            },
            CropStage {
                model: "::crate-color.glb".into(),
                duration: Range { min: 1, max: 2 },
                begin_status: None,
            },
            CropStage {
                model: "::crate-color.glb".into(),
                duration: Range { min: 1, max: 2 },
                begin_status: Some(CropStatus::Fruiting {
                    model: "::crate-color.glb".into(),
                    drops: vec![ItemDrop {
                        item: ItemId("test".into()),
                        amount: Range { min: 1, max: 3 },
                    }],
                }),
            },
        ],
    };
    match ron::ser::to_writer_pretty(
        file_ron,
        &item,
        PrettyConfig::default()
            .indentor("  ".into())
            .compact_arrays(true)
            .extensions(Extensions::all()),
    ) {
        Ok(_) => info!("Emitted example crop file to {}", path_ron),
        Err(err) => error!("Failed to emit example file: {} error: {}", path_ron, err),
    }
}

pub fn add_test_crop(mut cmd: Commands) {
    crop_from_asset("corn".into(), &mut cmd, Vec3::new(5., 0., 5.));
    crop_from_asset("beets".into(), &mut cmd, Vec3::new(-5., 0., 5.));
}

fn crop_from_asset(file: &str, cmd: &mut Commands, position: Vec3) {
    let gap = GameAssetPath::new_data(format!("::crops/{}.crop.ron", file));
    let Ok(reader) = File::open(gap.path_relative()) else {
        error!("File not found: {}", gap);
        return;
    };
    let def = match ron::de::from_reader::<_, CropDefinition>(reader) {
        // TODO: this should definitely be ported over to a more Asset based approach
        Ok(d) => d,
        Err(e) => {
            error!("Failed to deserialize data from {}, {}", file, e);
            return;
        }
    };

    let Ok(bundle) = CropBundle::new(def.stages) else {
        error!("Failed to construct a crop bundle from file");
        return;
    };
    cmd.spawn((
        Name::new(format!("{} - {:.1},{:.1}", def.id, position.x, position.z)),
        // Todo component `cleanup::{??}`
        SpatialBundle {
            transform: Transform::from_xyz(position.x, position.y, position.z)
                .with_scale(Vec3::ONE * 5.),
            ..default()
        },
        bundle,
    ));
}

pub fn initialize_crops(
    query: Query<(&CropData, Entity), Without<CropTimer>>, mut cmd: Commands,
    assets: Res<AssetServer>, mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    for (data, entity) in query.iter() {
        let Some(start) = data.stages.first().cloned() else {
            cmd.entity(entity).despawn_recursive();
            continue;
        };
        let scene = assets.load::<Scene>(GameAssetPath::new_model(start.model).gltf_scene());

        cmd.entity(entity).insert((
            start.begin_status.unwrap_or_default(),
            CropTimer(start.duration.get(&mut rng)),
            scene,
            RigidBody::Static,
            ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
        ));
    }
}

pub fn update_crops(
    _: Trigger<NewDay>,
    mut query: Query<(
        &mut CropData,
        &mut CropTimer,
        &GlobalTransform,
        &CropStatus,
        Option<&Name>,
        Entity,
    )>,
    mut cmd: Commands, mut rng: ResMut<GlobalEntropy<WyRand>>, assets: Res<AssetServer>,
    children_query: Query<&Children>,
) {
    for (mut data, mut timer, trans, last_status, name, entity) in query.iter_mut() {
        if let Some(safe_num) = timer.0.checked_sub(1) {
            // we did not try to subtract 1 from 0 (disallowed on u32)
            timer.0 = safe_num;
            return;
        }
        data.index += 1;
        let Some(stage) = data.stages.get(data.index) else {
            warn!("Crop reached end of cycle: {:} ({:?})", entity, name);
            cmd.entity(entity).despawn();
            return;
        };

        cmd.trigger(CropStageChange {
            entity,
            name: name.cloned(),
            position: trans.translation(),
        });

        for child in children_query.iter_descendants(entity) {
            // removing colliders allows for regeneration based on the changing scene
            cmd.entity(child).remove::<Collider>();
        }
        set_crop_model(&mut cmd.entity(entity), &stage.model, &assets);

        match last_status {
            CropStatus::Growing => (),
            CropStatus::Dead => cmd.entity(entity).despawn(),
            CropStatus::Seeding { model: _, drops } => {
                info!("Dropping items: {:#?}", drops);
            }
            CropStatus::Fruiting { model: _, drops } => {
                info!("Dropping items: {:#?}", drops);
            }
        }

        // Crop Status Stuff

        let Some(new_status) = &stage.begin_status else {
            return;
        };
        cmd.entity(entity).insert((new_status.clone(),));

        match new_status {
            CropStatus::Fruiting { model, drops } => {
                cmd.entity(entity).insert((CropFruit(drops.clone()),));
                set_crop_model(&mut cmd.entity(entity), model, &assets);
            }
            CropStatus::Seeding { model, drops } => {
                cmd.entity(entity).insert((CropFruit(drops.clone()),));
                set_crop_model(&mut cmd.entity(entity), model, &assets);
            }
            CropStatus::Dead => {
                cmd.entity(entity).despawn();
            }
            CropStatus::Growing => {
                cmd.entity(entity)
                    .insert((CropTimer(stage.duration.get(&mut rng)),));
            }
        }
    }
}

fn set_crop_model<S: Into<String> + Clone>(
    entity: &mut EntityCommands, path: &S, assets: &Res<AssetServer>,
) {
    entity.insert((
        assets.load::<Scene>(GameAssetPath::new_model(path.clone()).gltf_scene()),
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh)
            .with_default_density(ColliderDensity(1.0)),
    ));
}
