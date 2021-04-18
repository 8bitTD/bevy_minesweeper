use bevy::{prelude::*};
use super::super::state::*;

pub fn setup_gameover(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
) {
    commands.spawn_bundle(UiCameraBundle::default()).insert(ReleaseResource);
    commands.insert_resource(ClearColor(Color::rgb(0.00, 0.00, 0.00)));
    commands.spawn_bundle(ButtonBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(5.0),
                right: Val::Px(5.0),
                ..Default::default()
            },
            size: Size::new(Val::Px(100.0), Val::Px(30.0)),
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        material: button_materials.normal.clone(),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                "Title",
                TextStyle {
                    font: asset_server.load(font::E),
                    font_size: font::SIZE,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
                Default::default(),
            ),
            ..Default::default()
        });
    }).insert(ReleaseResource);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(ReleaseResource);
    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "State: GameOver",
            TextStyle {
                font: asset_server.load(font::E),
                font_size: 30.0,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(0.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }).insert(ReleaseResource);
}

pub fn update_gameover(
    mut state: ResMut<State<GameState>>,
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut material) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                state.set(GameState::Title).unwrap();
            }
            Interaction::Hovered => { *material = button_materials.hovered.clone(); }
            Interaction::None => { *material = button_materials.normal.clone(); }
        }
    }
}