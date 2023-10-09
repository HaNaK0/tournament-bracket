use bevy::{ecs::system::EntityCommands, prelude::*, utils::HashMap};

use crate::room::{Prefab, PrefabField, PrefabRegistry};

pub struct DefaultPrefabsPlugin;

impl Plugin for DefaultPrefabsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut registry: ResMut<PrefabRegistry>) {
    trace!("prefab plugin setup");
    registry.register_prefab("Player", PlayerPrefab::spawn_prfab);
    registry.register_prefab("PigParent", PigParentPrefab::spawn_prfab)
}

pub struct PlayerPrefab;

impl Prefab for PlayerPrefab {
    fn spawn_prfab(
        fields: &HashMap<String, PrefabField>,
        mut commands: EntityCommands,
        asset_server: &AssetServer,
    ) {
        commands.insert(Name::new("Player".to_string()));

        if let PrefabField::Number(speed) = fields
            .get("speed")
            .as_ref()
            .expect("No speed value in player")
        {
            commands.insert(crate::Player {
                speed: *speed as f32,
            });
        } else {
            warn!("Speed value har wrong type")
        }

        if let (PrefabField::String(sprite), PrefabField::String(position)) = (
            fields
                .get("sprite")
                .as_ref()
                .expect("No sprite field on player"),
            fields
                .get("position")
                .as_ref()
                .expect("No position field on player"),
        ) {
            let position: (f32, f32) =
                ron::de::from_str(position).expect("Failed parsing position");

            commands.insert(SpriteBundle {
                texture: asset_server.load(sprite),
                transform: Transform::from_translation(Vec3 {
                    x: position.0,
                    y: position.1,
                    z: 0.0,
                }),
                ..default()
            });
        } else {
            warn!("sprite or speed has wrong type");
        }
    }
}

pub struct PigParentPrefab;

impl Prefab for PigParentPrefab {
    fn spawn_prfab(
        _fields: &HashMap<String, PrefabField>,
        mut commands: EntityCommands,
        _asset_server: &AssetServer,
    ) {
        commands.insert((
            Name::new("PigParent"),
            crate::pig::PigParent {},
            SpatialBundle::default(),
        ));
    }
}
