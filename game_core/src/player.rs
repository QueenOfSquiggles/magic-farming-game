use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tnua::prelude::{
    TnuaBuiltinJump, TnuaBuiltinWalk, TnuaController, TnuaControllerBundle, TnuaControllerPlugin,
};
use bevy_tnua_avian3d::{TnuaAvian3dPlugin, TnuaAvian3dSensorShape};
use leafwing_input_manager::{
    plugin::InputManagerPlugin,
    prelude::{
        ActionState, GamepadStick, InputMap, KeyboardVirtualDPad, MouseMove,
        WithDualAxisProcessingPipelineExt,
    },
    Actionlike, InputControlKind, InputManagerBundle,
};

use crate::{
    collision::GameLayers,
    common_events::{PlayerInteract, PlayerInteractionChanged},
    interaction::Interactable,
    mouse::MouseState,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TnuaControllerPlugin::default(),
            TnuaAvian3dPlugin::default(),
            InputManagerPlugin::<InputActions>::default(),
        ));
        app.configure_sets(
            Update,
            SchedulePlayerMouseLocked.run_if(in_state(MouseState::Locked)),
        );
        app.add_systems(Startup, create_player);
        app.add_systems(
            Update,
            (
                camera_look,
                player_move,
                emit_interaction_events,
                dispatch_interactions,
            )
                .in_set(SchedulePlayerMouseLocked),
        );
    }
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
struct SchedulePlayerMouseLocked;

fn create_player(mut cmd: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let input = InputMap::<InputActions>::default()
        .with_dual_axis(InputActions::Move, KeyboardVirtualDPad::WASD)
        .with_dual_axis(InputActions::Move, KeyboardVirtualDPad::ARROW_KEYS)
        .with_dual_axis(
            InputActions::Move,
            GamepadStick::LEFT.with_circle_deadzone(0.1),
        )
        .with_dual_axis(InputActions::Look, MouseMove::default().inverted())
        .with_dual_axis(
            InputActions::Look,
            GamepadStick::RIGHT.with_circle_deadzone(0.1).inverted(),
        )
        .with(InputActions::Jump, KeyCode::Space)
        .with(InputActions::Primary, MouseButton::Left)
        .with(InputActions::Secondary, MouseButton::Right)
        .with(InputActions::Interact, KeyCode::KeyE)
        .with(InputActions::Jump, GamepadButtonType::East)
        .with(InputActions::Primary, GamepadButtonType::RightTrigger)
        .with(InputActions::Secondary, GamepadButtonType::LeftTrigger)
        .with(InputActions::Interact, GamepadButtonType::South);

    cmd.spawn((
        Name::new("Player"),
        PlayerMarker,
        RigidBody::Dynamic,
        Collider::capsule(0.5, 1.0),
        CollisionLayers::new(GameLayers::Player, LayerMask::ALL),
        TnuaControllerBundle::default(),
        LastInteractable::default(),
        TnuaAvian3dSensorShape(Collider::cylinder(0.49, 0.01)),
        // allow Y rotation for looking around
        LockedAxes::new().lock_rotation_x().lock_rotation_z(),
        InputManagerBundle::with_map(input),
        SpatialBundle {
            transform: Transform::from_xyz(0., 5.0, 0.),
            ..default()
        },
    ))
    .with_children(|b| {
        b.spawn(PbrBundle {
            mesh: meshes.add(Capsule3d {
                radius: 0.5,
                half_length: 0.75,
            }),
            ..default()
        });
        b.spawn((
            Name::new("PlayerCameraRoot"),
            FpsCameraRoot,
            SpatialBundle {
                transform: Transform::from_xyz(2., 0.75, 0.),
                ..default()
            },
        ))
        .with_children(|builder| {
            builder.spawn((
                Name::new("PlayerCamera"),
                FpsCameraMarker,
                Camera3dBundle {
                    projection: Projection::Perspective(PerspectiveProjection {
                        fov: 70.,
                        far: 300.,
                        ..default()
                    }),
                    transform: Transform::from_xyz(0.0, 0.0, 7.),
                    ..default()
                },
            ));
        });
    });
}

#[derive(Component)]
struct PlayerMarker;

#[derive(Component)]
struct FpsCameraMarker;

#[derive(Component)]
struct FpsCameraRoot;

#[derive(Component, Default, Debug)]
struct LastInteractable(Option<Entity>);

fn camera_look(
    mut cam_query: Query<&mut Transform, (With<FpsCameraRoot>, Without<PlayerMarker>)>,
    mut player_query: Query<
        (&mut Transform, &ActionState<InputActions>),
        (With<PlayerMarker>, Without<FpsCameraRoot>),
    >,
    time: Res<Time>,
) {
    let Ok(mut cam) = cam_query.get_single_mut() else {
        return;
    };
    let Ok((mut body, input)) = player_query.get_single_mut() else {
        return;
    };
    let look = input.axis_pair(&InputActions::Look);
    body.rotate_local_y(look.x * 45.0_f32.to_radians() * time.delta_seconds());
    cam.rotate(Quat::from_euler(
        EulerRot::XYZ,
        look.y * time.delta_seconds() * 45.0_f32.to_radians(),
        0.,
        0.,
    ));
    let vec = cam.rotation.to_euler(EulerRot::XYZ);
    cam.rotation = Quat::from_euler(
        EulerRot::XYZ,
        vec.0.clamp(-80_f32.to_radians(), 10_f32.to_radians()),
        vec.1,
        vec.2,
    );
}

fn player_move(
    mut controller_query: Query<
        (&mut TnuaController, &ActionState<InputActions>),
        (With<PlayerMarker>, Without<FpsCameraRoot>),
    >,
    cam_query: Query<&GlobalTransform, (With<FpsCameraRoot>, Without<PlayerMarker>)>,
) {
    let Ok((mut controller, input)) = controller_query.get_single_mut() else {
        return;
    };
    let Ok(cam) = cam_query.get_single() else {
        return;
    };
    // prefer clamp length rather than normalized to allow variance in speed
    let movement = input.axis_pair(&InputActions::Move).clamp_length_max(1.0);

    let basis_neg_z = cam.forward().normalize_or_zero();
    let basis_pos_x = cam.right().normalize_or_zero();

    // clear Y motion to avoid issues when looking up/down
    let desired = (basis_neg_z * movement.y + basis_pos_x * movement.x) * Vec3::new(1., 0., 1.);
    controller.basis(TnuaBuiltinWalk {
        desired_velocity: desired.normalize_or_zero() * 10.0,
        float_height: 1.5,
        ..default()
    });
    if input.pressed(&InputActions::Jump) {
        controller.action(TnuaBuiltinJump {
            height: 4.,
            ..default()
        });
    }

    // https://github.com/idanarye/bevy-tnua/blob/3562f35ef7c186c84d19b4855f8d3452e397839f/examples/example.rs
}

fn emit_interaction_events(
    mut interact_query: Query<&mut LastInteractable, With<PlayerMarker>>,
    root_query: Query<&GlobalTransform, With<FpsCameraRoot>>, spatial_query: SpatialQuery,
    parent_query: Query<&Parent>, interactable_query: Query<&Interactable>, mut commands: Commands,
) {
    let Ok(mut interact) = interact_query.get_single_mut() else {
        warn!("No last interactable component on player!");
        return;
    };
    let Ok(root) = root_query.get_single() else {
        warn!("No global transform on FPS Camera Root");
        return;
    };
    // try hit with raycast
    let ray_data = spatial_query.cast_ray(
        root.translation(),
        root.forward(),
        50.0,
        false,
        SpatialQueryFilter::from_mask(GameLayers::Interactable),
    );
    let hit_data: Option<Entity> = if let Some(data) = ray_data {
        // raycast hit success, YAY we have a (relatively) cheap and accurate hit
        Some(data.entity)
    } else {
        // Oh no, nothing there, let's try a shape cast just to be sure they're not like
        // 2 pixels off
        let shape_data = spatial_query.cast_shape(
            &Collider::sphere(0.5),
            root.translation(),
            Quat::default(),
            root.forward(),
            50.0,
            true,
            SpatialQueryFilter::from_mask(GameLayers::Interactable),
        );
        // composite into needed data
        shape_data.and_then(|data| Some(data.entity))
    };
    // Thisa effectively bubbles to the appropriate entity
    let hit_data = hit_data.and_then(|e| {
        // processes a valid collider entity into the first ancestor that contains an `Interactable` marker component
        if interactable_query.contains(e) {
            // For the edge case where the collider is on the root object (mainly only likely in testing and development)
            return Some(e);
        }
        for p in parent_query.iter_ancestors(e) {
            if interactable_query.contains(p) {
                return Some(p);
            }
        }
        None
    });

    // now we can check against possible cases where the
    if hit_data != interact.0 {
        interact.0 = hit_data;
        if let Some(target) = hit_data {
            commands.trigger_targets(PlayerInteractionChanged(hit_data), target);
        } else {
            commands.trigger(PlayerInteractionChanged(hit_data));
        }
    }
}

fn dispatch_interactions(
    query: Query<(&LastInteractable, &ActionState<InputActions>), With<PlayerMarker>>,
    mut cmd: Commands,
) {
    let Ok((last, input)) = query.get_single() else {
        return;
    };
    let Some(target) = last.0 else {
        return;
    };
    if !input.just_pressed(&InputActions::Interact) {
        return;
    }
    cmd.trigger_targets(PlayerInteract, target);
    info!("Player dispatched interaction event");
}

#[derive(Reflect, Clone, PartialEq, Eq, Hash, Debug)]
pub enum InputActions {
    Move,
    Look,
    Jump,
    Interact,
    Primary,
    Secondary,
    Cancel,
}

impl Actionlike for InputActions {
    fn input_control_kind(&self) -> InputControlKind {
        match *self {
            InputActions::Move => InputControlKind::DualAxis,
            InputActions::Look => InputControlKind::DualAxis,
            InputActions::Cancel => InputControlKind::Button,
            InputActions::Jump => InputControlKind::Button,
            InputActions::Primary => InputControlKind::Button,
            InputActions::Secondary => InputControlKind::Button,
            InputActions::Interact => InputControlKind::Button,
        }
    }
}
