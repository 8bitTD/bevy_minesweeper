use bevy::prelude::*;
use define::*;
pub mod state;
pub mod define;
fn main() {
    unsafe { kernel32::FreeConsole() };
    App::build()
    .insert_resource(WindowDescriptor{
        title: "bevy_template".to_string(),
        width: system::RESOLUTION,
        height: system::RESOLUTION,
        vsync: false,
        resizable: true,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(state::StatePlugin)
    .run();
}
