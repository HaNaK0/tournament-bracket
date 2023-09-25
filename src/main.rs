use bevy::{prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window { 
                    resolution: (640.0, 480.0).into(), 
                    resizable: false,
                    title: "rustiant".to_string(), 
                    ..default()
                }),
                ..default()
            })
        )
        .add_systems(Startup, setup)
        .add_systems(Update, movement_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        sprite: Sprite { 
            custom_size: (Some(Vec2::new(100.0, 100.0))),
            ..default()
        },
        texture : asset_server.load("bevy-icon.png"),
        ..default()
    });
}

fn movement_system(
    mut transforms: Query<&mut Transform, With<Sprite>>, 
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
)
    {
        for mut transform in &mut transforms {
            if keyboard_input.pressed(KeyCode::A) {
                transform.translation.x -= 100.0 * time.delta_seconds();
            }
            if keyboard_input.pressed(KeyCode::D) {
                transform.translation.x += 100.0 * time.delta_seconds();
            }
            if keyboard_input.pressed(KeyCode::W) {
                transform.translation.y += 100.0 * time.delta_seconds();
            }
            if keyboard_input.pressed(KeyCode::S) {
                transform.translation.y -= 100.0 * time.delta_seconds();
            }
        }
}
