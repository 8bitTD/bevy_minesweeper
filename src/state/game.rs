use bevy::{prelude::*};
use super::super::state::*;
use rand::Rng;
pub struct Timer{//fps計測用のタイマー　0.2秒ごとに更新するため
    pub count: f32,
}
#[derive(Debug, Copy, Clone)]
pub struct ButtonInfo{
    pub number: u8,//ボタンクリック後に表示する数値 0 ~ 9
    pub x: usize,
    pub y: usize,
    pub tx: f32,
    pub ty: f32,
    pub is_ext: bool,
}

impl ButtonInfo{//ボタンの情報
    pub fn new() -> ButtonInfo {
        ButtonInfo {number: 0,x: 0,y: 0,tx: 0.0,ty: 0.0, is_ext: true}
    }
    pub fn set(_number: u8, _x: usize, _y: usize, _tx: f32, _ty: f32, _is_ext: bool) -> ButtonInfo {
        ButtonInfo {number: _number, x: _x, y: _y, tx: _tx, ty: _ty, is_ext: _is_ext}
    }
}
pub struct ButtonInfos{//全ボタンの情報
    pub info: [[ButtonInfo; button::LINE]; button::LINE],
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ButtonPush{
    pub x: usize,
    pub y: usize,
}
impl ButtonPush{//ボタンの情報
    pub fn set(_x: usize, _y: usize) -> ButtonPush {
        ButtonPush {x: _x, y: _y}
    }
}

pub struct FpsText;//fps計測用のテキスト
pub struct ClearText;//クリア― 計測用のテキスト
pub struct ClearCount{//クリアー 計測用のカウント　count / all
    pub count: i32,
    pub all: i32,
}

pub fn fps(//fps計測する処理
    mut query: Query<&mut Text, With<FpsText>>,
    mut counter: ResMut<Timer>,
    time: Res<Time>, 
){
    counter.count += time.delta_seconds();
    if counter.count < 0.2 {return;}
    let fps = (1.0 / time.delta_seconds()) as i32;
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{0}: {1}","Fps", fps);
    }
    counter.count = 0.0;
}

pub fn get_zero_button(//隣接する0ボタンを取得する再帰処理
    x: usize,
    y: usize,
    btnis: &mut ResMut<ButtonInfos>,
    bps: &mut Vec<ButtonPush>,
){
    for xx in 0..3{
        let vx = x + xx -1;
        if  vx > button::LINE-1 {continue;}
        for yy in 0..3{
            let vy = y + yy -1;
            if vy > button::LINE-1 {continue;}
            if xx == 1 && yy == 1 {continue;}
            if btnis.info[vx][vy].number != 9 && btnis.info[vx][vy].is_ext == true && !bps.contains(&ButtonPush::set(vx, vy)){
                bps.push(ButtonPush::set(vx, vy));
                if btnis.info[vx][vy].number == 0 {
                    get_zero_button(vx, vy, btnis, bps);
                }
            }
        }
    }
}

fn push_button(//指定したボタンを押す処理
    commands: &mut Commands,
    x: usize,
    y: usize,
    clear: &mut ClearCount,
    materials: &mut Assets<ColorMaterial>,
    asset_server: &AssetServer,
    btn_query: &mut Query<(Entity, &ButtonInfo)>,
    btnis: &mut ResMut<ButtonInfos>,
){
    for (ett, btni) in btn_query.iter_mut() {
        if btni.x != x || btni.y != y {continue;}
        commands.entity(ett).despawn_recursive();
        commands.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(button::SIZE-2.0), Val::Px(button::SIZE-2.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px( btni.ty - (button::SIZE*0.5 as f32)+1.0),
                    left: Val::Px(btni.tx - (button::SIZE*0.5 as f32)+1.0),
                    ..Default::default()
                },
                border: Rect::all(Val::Px(20.0)),
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            ..Default::default()
        }).insert(ReleaseResource)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                btni.number.to_string(),
                TextStyle {
                    font: asset_server.load(font::E),
                    font_size: button::SIZE,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
                Default::default(),
            ),
            style: Style {
                size: Size::new(Val::Px(button::SIZE), Val::Px(button::SIZE)),
                position_type: PositionType::Relative,
                align_self: AlignSelf::Center,
                position: Rect {
                    bottom: Val::Auto,
                    right: Val::Px(7.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }).insert(ReleaseResource); });
        btnis.info[x][y].is_ext = false;
        clear.count += 1;
    }   
} 

pub fn update_button(//Minesweeper用のボタンUpdate処理
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut interaction_query: Query< (&Interaction, &mut Handle<ColorMaterial>,  &ButtonInfo)>,
    mut query: Query<&mut Text, With<ClearText>>,
    mut clear: ResMut<ClearCount>,
    mut btn_query: Query<(Entity, &ButtonInfo)>,
    mut btnis: ResMut<ButtonInfos>,
    button_materials: Res<ButtonMaterials>,
){
    for (interaction, mut material,btni) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                if btni.number == 9 { 
                    state.set(GameState::GameOver).unwrap();
                    return;
                }
                *material = button_materials.pressed.clone();
                push_button(&mut commands, btni.x, btni.y, &mut clear, &mut materials, &asset_server, &mut btn_query, &mut btnis);
                if btni.number == 0{
                    let mut bps:Vec<ButtonPush> = Vec::new();
                    get_zero_button(btni.x, btni.y,  &mut btnis, &mut bps);
                    for b in bps{
                        push_button(&mut commands, b.x, b.y, &mut clear, &mut materials, &asset_server, &mut btn_query, &mut btnis);
                    }
                }
                
                for mut text in query.iter_mut() {
                    text.sections[0].value = format!("{0}/{1}",clear.count, clear.all);
                }
                if clear.count == clear.all{state.set(GameState::Ending).unwrap();}
           }
            Interaction::Hovered => { *material = button_materials.hovered.clone(); }
            Interaction::None => {*material = button_materials.normal.clone();}
        }
    }
}

fn edit_clear_setting(//クリア設定を編集
    clear: &mut ResMut<ClearCount>,
    btnis: &mut ResMut<ButtonInfos>,
) {
    clear.count = 0;
    clear.all = 0;
    let mut num  =  [[0 as u8; button::LINE]; button::LINE];
    for y in 0..button::LINE {
        for x in 0..button::LINE {
            let rnd:u8 = rand::thread_rng().gen_range(0..10);
            num[x][y] = rnd;
        }
    }
    for y in 0..button::LINE {
        for x in 0..button::LINE {
            let mut cnt = 0;
            for xx in 0..3{
                let xv:u8 = (x+xx-1) as u8;
                if xv < 0 as u8 || xv > (button::LINE-1) as u8 {continue;}
                for yy in 0..3{
                    let yv:u8 = (y+yy-1) as u8;
                    if yv < 0 as u8 || yv > (button::LINE-1) as u8 {continue;}
                    if num[xv as usize][yv as usize] == 9 {cnt +=1;}
                }
            }
            if num[x][y] == 9{cnt = 9;}
            let txx: f32 = x as f32*(button::SIZE-1.0)+button::SIZE*1.5;
            let tyy: f32 = y as f32*(button::SIZE-1.0)+button::SIZE*1.5;
            btnis.info[x][y] = ButtonInfo::set(cnt, x, y, txx, tyy, true);
            if num[x][y] != 9 {clear.all += 1;}
        }
    }
}

pub fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterials>,
    mut clear: ResMut<ClearCount>,
    mut btnis: ResMut<ButtonInfos>,
) {
    clear.count = 0;
    clear.all = 0;
    btnis.info = [[ButtonInfo::new(); button::LINE]; button::LINE];
    edit_clear_setting(&mut clear, &mut btnis);//クリア設定を編集
    commands.insert_resource(ClearColor(Color::rgb(0.40, 0.40, 0.40)));
    commands.spawn_bundle(UiCameraBundle::default()).insert(ReleaseResource);
    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            format!(r"{0}/{1}", clear.count, clear.all),
            TextStyle {
                font: asset_server.load(font::E),
                font_size: 30.0,
                color: Color::rgb(1.0, 1.0, 1.0),
            },
            Default::default(),
        ),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(0.0),
                left: Val::Percent(45.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }).insert(ReleaseResource).insert(ClearText);

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
    }).with_children(|parent| {
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
    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "State: Game",
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
    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "Fps:",
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
                bottom: Val::Px(0.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }).insert(FpsText).insert(ReleaseResource);

    for y in 0..button::LINE {
        for x in 0..button::LINE {
            let col = Color::rgb(button::NORMAL.0, button::NORMAL.1, button::NORMAL.2).into();
            commands.spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(button::SIZE), Val::Px(button::SIZE)),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    position: Rect {
                        top: Val::Px((button::SIZE-1.0)*(y as f32)+button::SIZE),
                        left: Val::Px((button::SIZE-1.0)*(x as f32)+button::SIZE),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                material: materials.add(col),
                transform: Transform::from_scale(Vec3::splat(0.95)),
                ..Default::default()
            }).insert(ReleaseResource).insert(ButtonInfo{
                number: btnis.info[x][y].number,
                x: btnis.info[x][y].x,
                y: btnis.info[x][y].y,
                tx: btnis.info[x][y].tx,
                ty: btnis.info[x][y].ty,
                is_ext: true,
            });
            commands.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "",
                    TextStyle {
                        font: asset_server.load(font::E),
                        font_size: 30.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                style: Style {
                    size: Size::new(Val::Px(button::SIZE), Val::Px(button::SIZE)),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    position: Rect {
                        top: Val::Px((button::SIZE-1.0)*(y as f32)+button::SIZE),
                        left: Val::Px((button::SIZE-1.0)*(x as f32)+button::SIZE),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            }).insert(ReleaseResource);
        }
    }
}

pub fn update_game(
    mut state: ResMut<State<GameState>>,
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut material,children) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                let text = &text_query.get_mut(children[0]).unwrap().sections[0].value;
                if text == "Title"{ state.set(GameState::Title).unwrap(); }
                *material = button_materials.pressed.clone();
            }
            Interaction::Hovered => { *material = button_materials.hovered.clone(); }
            Interaction::None => {*material = button_materials.normal.clone(); }
        }
    }
}