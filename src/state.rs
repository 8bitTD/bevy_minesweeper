use bevy::{prelude::*};
use super::define::*;
pub mod title;
pub mod game;
pub mod gameover;
pub mod ending;
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Title,
    Game,
    GameOver,
    Ending,
}
pub struct ReleaseResource;
pub fn appstate_exit_despawn(mut commands: Commands, query: Query<(&ReleaseResource, Entity)>) {
    for (_,entity) in &mut query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build (&self, app: &mut AppBuilder){
        app
            .init_resource::<ButtonMaterials>()
            .insert_resource(game::Timer { count: 0.0 })
            .insert_resource(game::ClearCount { count: 0, all: 0 })
            .insert_resource(game::ButtonInfo {number:0, x: 0, y: 0, tx: 0.0, ty: 0.0, is_ext: true})
            .insert_resource(game::ButtonInfos {info:  [[game::ButtonInfo::new(); button::LINE]; button::LINE] })
            .insert_resource(game::ButtonPush {x: 0, y: 0})
            .insert_resource(ending::MoveText { up_value: 0.0, up_offset: 0.0})
            .add_state(GameState::Title)
            .add_system_set(SystemSet::on_enter(GameState::Title).with_system(title::setup_title.system()))
            .add_system_set(SystemSet::on_update(GameState::Title).with_system(title::update_title.system()))
            .add_system_set(SystemSet::on_exit(GameState::Title).with_system(appstate_exit_despawn.system()))

            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(game::setup_game.system()))
            .add_system_set(SystemSet::on_update(GameState::Game).with_system(game::update_button.system()))
            .add_system_set(SystemSet::on_update(GameState::Game).with_system(game::update_game.system()))
            .add_system_set(SystemSet::on_update(GameState::Game).with_system(game::fps.system()))
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(appstate_exit_despawn.system()))

            .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(gameover::setup_gameover.system()))
            .add_system_set(SystemSet::on_update(GameState::GameOver).with_system(gameover::update_gameover.system()))
            .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(appstate_exit_despawn.system()))

            .add_system_set(SystemSet::on_enter(GameState::Ending).with_system(ending::setup_ending.system()))
            .add_system_set(SystemSet::on_update(GameState::Ending).with_system(ending::update_ending.system()))
            .add_system_set(SystemSet::on_update(GameState::Ending).with_system(ending::animate.system()))
            .add_system_set(SystemSet::on_exit(GameState::Ending).with_system(appstate_exit_despawn.system()))

            .run();
    }
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(button::NORMAL.0, button::NORMAL.1, button::NORMAL.2).into()),
            hovered: materials.add(Color::rgb(button::HOVERED.0, button::HOVERED.1, button::HOVERED.2).into()),
            pressed: materials.add(Color::rgb(button::PRESSED.0, button::PRESSED.1, button::PRESSED.2).into()),
        }
    }
}
pub struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}