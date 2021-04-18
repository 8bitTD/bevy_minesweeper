pub mod button{
    pub const PRESSED: (f32, f32, f32) = (0.35, 0.75, 0.35);
    pub const HOVERED: (f32, f32, f32) = (0.25, 0.25, 0.25);
    pub const NORMAL: (f32, f32, f32) = (0.15, 0.15, 0.15);
    pub const LINE: usize = 15;
    pub const SIZE: f32 = 50.0;
}
pub mod font{
    pub const E: &str = "fonts/FiraSans-Bold.ttf";
    pub const J: &str = "fonts/NotoSansCJKjp-Regular.otf";
    pub const SIZE: f32 = 30.0;
}
pub mod system{
    pub const RESOLUTION: f32 = 850.0;
    pub const FPS: f32 = 60.0;
    pub const SPAWN: i32 = 200;
}
pub mod credit {
    pub const ENDING_TEXT: &str = 
    "
    ゲームデザイン
    - - - -

    ゲームプログラム
    - - - -

    ゲームグラフィック
    - - - -

    










    Thank you for playing! 
    ";
}
