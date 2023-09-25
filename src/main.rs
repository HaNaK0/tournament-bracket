use bevy::{prelude::*, render::camera::ScalingMode};

#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
struct Pig {
    lifetime: Timer,
}

#[derive(Resource)]
struct Money(f32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (640.0, 480.0).into(),
                resizable: true,
                title: "rustiant".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (movement_system, spawn_pigs, update_pig_system))
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

fn spawn_pigs(
    mut commands: Commands,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) || money.0 < 10.0 {
        return;
    }

    money.0 -= 10.0;

    let texture = asset_server.load("sprites/Animals/pig.png");
    let player_transform = player.single();

    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_translation(player_transform.translation),
            ..default()
        },
        Pig {
            lifetime: Timer::from_seconds(2.0, TimerMode::Once),
        },
    ));
}

fn update_pig_system(
    mut commands: Commands,
    mut pigs: Query<(Entity, &mut Pig)>,
    mut money: ResMut<Money>,
    time: Res<Time>,
) {
    for (entity, mut pig) in &mut pigs {
        pig.lifetime.tick(time.delta());

        if pig.lifetime.finished() {
            commands.entity(entity).despawn();
            money.0 += 15.0;
        }
    }
}
