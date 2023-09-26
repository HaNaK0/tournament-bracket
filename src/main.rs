use bevy::{
    input::common_conditions::input_toggle_active, prelude::*, render::camera::ScalingMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use pig::PigPlugin;
use ui::GameUiPlugin;

mod pig;
mod ui;

#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Resource)]
struct Money(f32);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (640.0, 480.0).into(),
                    resizable: true,
                    title: "rustiant".to_string(),
                    ..default()
                }),
                ..default()
            }),
            PigPlugin,
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::F3)),
            GameUiPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, movement_system)
        .insert_resource(Money(100.0))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 1920.0,
        min_height: 1080.,
    };

    commands.spawn(camera);

    let texture = asset_server.load("sprites/bevy-icon.png");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite { ..default() },
            texture,
            ..default()
        },
        Player { speed: 300.0 },
        Name::new("Player"),
    ));
}

fn movement_system(
    mut players: Query<(&mut Transform, &Player)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut players {
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= player.speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += player.speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += player.speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= player.speed * time.delta_seconds();
        }
    }
}
