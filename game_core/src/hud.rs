use bevy::prelude::*;

use crate::common_events::PlayerInteractionChanged;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_hud);
        app.observe(update_interact_label);
    }
}

#[derive(Component)]
struct InteractNameLabel;

fn create_hud(mut cmd: Commands) {
    // cmd.spawn(UI)
    cmd.spawn((
        Name::new("Interaction Name Label"),
        InteractNameLabel,
        TextBundle::from_section("---", TextStyle::default()).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(32.),
            right: Val::Px(32.),

            ..default()
        }),
    ));
}

fn update_interact_label(
    trigger: Trigger<PlayerInteractionChanged>,
    mut query: Query<&mut Text, With<InteractNameLabel>>, name_query: Query<&Name>,
) {
    let Ok(mut label) = query.get_single_mut() else {
        warn!("Couldn't find HUD label");
        return;
    };

    let Some(entity) = trigger.event().0 else {
        label.sections[0].value = "---".into();
        return;
    };
    let name = if let Ok(name) = name_query.get(entity) {
        name.as_str()
    } else {
        "Unnamed entity!"
    };
    label.sections[0].value = name.into();
}
