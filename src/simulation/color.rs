#[macro_export]
macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        [$r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0, 1.0]
    };

    ($r:expr, $g:expr, $b:expr, $a: expr) => {
        [$r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0, $a]
    };

    (f32 $r:expr, $g:expr, $b:expr) => {
        [$r, $g, $b, 1.0]
    };

    (f32 $r:expr, $g:expr, $b:expr, $a: expr) => {
        [$r, $g, $b, $a]
    };
}

pub type Color = [f32; 4];

pub const WHITE: Color = rgb!(255, 255, 255);
pub const BLACK: Color = rgb!(0, 0, 0);
pub const RED: Color = rgb!(255, 0, 0);
pub const BLUE: Color = rgb!(0, 0, 255);
pub const GREEN: Color = rgb!(0, 255, 0);
