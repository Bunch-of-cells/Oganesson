// use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const STEP: f32 = 0.2;

fn main() {
    let mut y = 0.0;
    let f = |t: f32| [0.0; 100].map(|_| rand::random::<i8>() as f32)[(t * 20.0) as usize];
    // let f = f32::sin;
    for t in 0..100 {
        let t = t as f32 / 20.0;
        let y13 = y + simpson13(t, f);
        let y38 = y + simpson38(t, f);
        let y_e = y + t * f(t + STEP);
        y = y38;
        println!("{:<12} :: {:<12} :: {:<12} : {}", y13, y38, y_e, t);
    }
}

fn simpson13(t: f32, f: fn(f32) -> f32) -> f32 {
    (f(t) + 4.0 * f(t + STEP) + f(t + 2.0 * STEP)) * STEP / 3.0
}

fn simpson38(t: f32, f: fn(f32) -> f32) -> f32 {
    (f(t) + 3.0 * f(t + STEP) + 3.0 * f(t + 2.0 * STEP) + f(t + 3.0 * STEP)) * 3.0 * STEP / 8.0
}

// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     commands.spawn_bundle(Camera2dBundle::default());

//     // Rectangle
//     commands.spawn_bundle(SpriteBundle {
//         sprite: Sprite {
//             color: Color::rgb(0.25, 0.25, 0.75),
//             custom_size: Some(Vec2::new(50.0, 100.0)),
//             ..default()
//         },
//         ..default()
//     });

//     // Circle
//     commands.spawn_bundle(MaterialMesh2dBundle {
//         mesh: meshes.add(shape::Circle::new(50.).into()).into(),
//         material: materials.add(ColorMaterial::from(Color::PURPLE)),
//         transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
//         ..default()
//     });

//     // Hexagon
//     commands.spawn_bundle(MaterialMesh2dBundle {
//         mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
//         material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
//         transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
//         ..default()
//     });
// }
