use bevy::prelude::*;

pub struct GameUiPlugin;

#[derive(Component)]
struct MoneyText;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui_system);
    }
}

fn spawn_ui_system(mut _commands: Commands) {
    
}
