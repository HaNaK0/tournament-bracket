use bevy::{
    asset::{AssetLoader, LoadedAsset},
    ecs::system::EntityCommands,
    prelude::*,
    reflect::{TypePath, TypeUuid},
    utils::HashMap,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Room>();
        app.add_event::<LoadRoomEvent>();
        app.init_resource::<PrefabRegistry>();
        app.init_resource::<RoomTracker>();
        app.add_asset_loader(RoomLoader);
        app.add_systems(Update, (load_room_system, parse_room_system));
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrefabData {
    #[serde(rename = "type")]
    pub prefab_type: String,
    pub fields: HashMap<String, PrefabField>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PrefabField {
    String(String),
    Number(f64),
    Bool(bool),
}

pub trait Prefab {
    fn spawn_prfab(
        fields: &HashMap<String, PrefabField>,
        commands: EntityCommands,
        asset_server: &AssetServer,
    );
}

#[derive(Deserialize, TypePath, TypeUuid, Debug)]
#[uuid = "23c6bd8f-a194-43f3-93be-9a3f95354c7f"]
pub struct Room {
    prefabs: Vec<PrefabData>,
}

/// The assetloader
#[derive(Default)]
pub struct RoomLoader;

impl AssetLoader for RoomLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let room = ron::de::from_bytes::<Room>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(room));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron", "room"]
    }
}

/// Event used to cue a room to be loaded
#[derive(Event, Debug, Default)]
pub struct LoadRoomEvent {
    file_path: PathBuf,
}

impl LoadRoomEvent {
    pub fn new(path: PathBuf) -> Self {
        LoadRoomEvent { file_path: path }
    }
}

/// Tracks which rooms are currently being loaded.
#[derive(Resource, Default)]
struct RoomTracker {
    rooms: Vec<RoomLoadingStatus>,
}

// Enum that keeps track of current status of a room being loaded
#[derive(Debug, Clone)]
enum RoomLoadingStatus {
    Loading(Handle<Room>),
    Parsed(Handle<Room>),
}

impl RoomLoadingStatus {
    #[allow(dead_code)]
    fn is_parsed(&self) -> bool {
        match self {
            RoomLoadingStatus::Loading(_) => false,
            RoomLoadingStatus::Parsed(_) => true,
        }
    }
}

/// Checks for a room load event and ques that room asset for loading
fn load_room_system(
    asset_server: Res<AssetServer>,
    mut load_event: EventReader<LoadRoomEvent>,
    mut room_tracker: ResMut<RoomTracker>,
) {
    for event in load_event.iter() {
        debug!("Queing room at path {:?} to be loaded", event.file_path);
        room_tracker.rooms.push(RoomLoadingStatus::Loading(
            asset_server.load(event.file_path.clone()),
        ));
    }
}

/// Checks wether a room asset has been loaded and the parses that room asset and spawns the prefab it has
fn parse_room_system(
    mut commands: Commands,
    registry: Res<PrefabRegistry>,
    mut room_tracker: ResMut<RoomTracker>,
    room_assets: Res<Assets<Room>>,
    asset_server: Res<AssetServer>,
) {
    room_tracker.rooms = room_tracker
        .rooms
        .iter()
        .map(|room_status| {
            if let RoomLoadingStatus::Loading(room_handle) = room_status {
                if let Some(room) = room_assets.get(room_handle) {
                    debug!("Parsing room {:?} : {:?}", room_handle, room);
                    for prefab in &room.prefabs {
                        let entity = commands.spawn_empty();
                        registry.spawn(&prefab, entity, &asset_server)
                    }
                    RoomLoadingStatus::Parsed(room_handle.clone())
                } else {
                    room_status.clone()
                }
            } else {
                room_status.clone()
            }
        })
        .collect()
}

/// A struct that tracks the spawn functions for all available prefabs
#[derive(Default, Resource)]
pub struct PrefabRegistry {
    prefabs: HashMap<
        String,
        Box<dyn Fn(&HashMap<String, PrefabField>, EntityCommands, &AssetServer) + Send + Sync>,
    >,
}

impl PrefabRegistry {
    /// Register a prefab to the registry
    pub fn register_prefab(
        &mut self,
        name: &str,
        spawn_fn: impl Fn(&HashMap<String, PrefabField>, EntityCommands, &AssetServer)
            + 'static
            + Send
            + Sync,
    ) {
        self.prefabs.insert(name.to_string(), Box::new(spawn_fn));
    }

    /// Calls the correct spawn function for a prefab of given type
    pub fn spawn(
        &self,
        prefab_data: &PrefabData,
        commands: EntityCommands,
        asset_server: &AssetServer,
    ) {
        self.prefabs[&prefab_data.prefab_type](&prefab_data.fields, commands, asset_server)
    }
}
