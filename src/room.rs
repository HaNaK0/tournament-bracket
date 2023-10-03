use bevy::{ecs::system::EntityCommands, prelude::*, utils::HashMap};
use serde::{Deserialize, Serialize};

use crate::Player;

pub struct RoomPlugin;

#[derive(Serialize, Deserialize)]
pub struct Room {
    prefabs: Vec<PrefabData>,
}

impl Room {
    fn generate_test_room() -> Self {
        let player_fields = [
            ("speed".to_string(), Some(PrefabField::Number(300.0))),
            (
                "position".to_string(),
                Some(PrefabField::String("(0.0, 0.0)".to_string())),
            ),
            (
                "sprite".to_string(),
                Some(PrefabField::String("sprites\\bevy-icon.png".to_string())),
            ),
        ];

        let player_prefab = PrefabData {
            prefab_type: "Player".to_string(),
            fields: HashMap::from_iter(player_fields.into_iter()),
        };

        let pig_parent_prefab = PrefabData {
            prefab_type: "PigParent".to_string(),
            fields: HashMap::new(),
        };

        Room {
            prefabs: vec![player_prefab, pig_parent_prefab],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PrefabData {
    prefab_type: String,
    fields: HashMap<String, Option<PrefabField>>,
}

#[derive(Serialize, Deserialize)]
pub enum PrefabField {
    String(String),
    Number(f64),
    Bool(bool),
}

pub trait Prefab {
    fn spawn_prfab(
        fields: HashMap<String, Option<PrefabField>>,
        commands: EntityCommands,
        asset_server: &AssetServer,
    );
}

struct PlayerPrefab;

impl Prefab for PlayerPrefab {
    fn spawn_prfab(
        fields: HashMap<String, Option<PrefabField>>,
        mut commands: EntityCommands,
        asset_server: &AssetServer,
    ) {
        commands.insert(Name::new("Player".to_string()));

        if let PrefabField::Number(speed) =
            fields["speed"].as_ref().expect("No speed value in player")
        {
            commands.insert(Player {
                speed: *speed as f32,
            });
        } else {
            warn!("Speed value har wrong type")
        }

        if let (PrefabField::String(sprite), PrefabField::String(position)) = (
            fields["sprite"]
                .as_ref()
                .expect("No sprite field on player"),
            fields["position"]
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

struct PigParentPrefab;

impl Prefab for PigParentPrefab {
    fn spawn_prfab(
        _fields: HashMap<String, Option<PrefabField>>,
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

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        let mut registry = PrefabRegistry::default();

        registry.register_prefab("Player", PlayerPrefab::spawn_prfab);
        registry.register_prefab("PigParent", PigParentPrefab::spawn_prfab);

        app.insert_resource(registry);

        app.add_systems(Startup, load_room);
    }
}

#[derive(Default, Resource)]
pub struct PrefabRegistry {
    prefabs: HashMap<
        String,
        Box<
            dyn Fn(HashMap<String, Option<PrefabField>>, EntityCommands, &AssetServer)
                + Send
                + Sync,
        >,
    >,
}

impl PrefabRegistry {
    fn register_prefab(
        &mut self,
        name: &str,
        spawn_fn: impl Fn(HashMap<String, Option<PrefabField>>, EntityCommands, &AssetServer)
            + 'static
            + Send
            + Sync,
    ) {
        self.prefabs.insert(name.to_string(), Box::new(spawn_fn));
    }
}

fn load_room(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    registry: Res<PrefabRegistry>,
) {
    let room = Room::generate_test_room();

    for prefab_data in room.prefabs {
        registry.prefabs[&prefab_data.prefab_type](
            prefab_data.fields,
            commands.spawn_empty(),
            &asset_server,
        )
    }
}
