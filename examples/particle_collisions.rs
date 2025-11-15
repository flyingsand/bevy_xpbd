use bevy::prelude::*;
use bevy_xpbd::*;
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugins(XpbdPlugins)
        .add_systems(Startup, startup)
        .insert_resource(Gravity(Vec2::ZERO))
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let sphere = meshes.add(Circle::new(50.0));

    let white = materials.add(Color::WHITE);

    commands
        .spawn((
            Mesh2d(sphere.clone()),
            MeshMaterial2d(white),
            Transform::default(),
        ))
        .insert(ParticleBundle::new_with_pos_and_vel(
            Vec2::new(60., 0.),
            Vec2::new(-60., 0.),
        ))
        .insert(Mass(1.));
    commands
        .spawn((
            Mesh2d(sphere.clone()),
            MeshMaterial2d(materials.add(Color::hsl(156., 1., 0.5))),
            Transform::default(),
        ))
        .insert(ParticleBundle::new_with_pos_and_vel(
            Vec2::new(-60., 0.),
            Vec2::new(20., 0.),
        ))
        .insert(Mass(5.));
    commands.spawn(Camera2d);
}
