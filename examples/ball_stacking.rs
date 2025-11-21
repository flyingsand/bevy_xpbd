use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_xpbd::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.8, 0.8, 0.9)))
        .insert_resource(Time::from_hz((1. / SUB_DT).into()))
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(XpbdPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_balls)
        //.add_systems(FixedUpdate, spawn_marbles)
        // .add_systems(Update, despawn_marbles)
        .run();
}

fn spawn_camera(mut command: Commands) {
    command.spawn((
        Camera2d,
        Transform::from_translation(Vec3::new(0., 0., 100.)),
        Projection::Orthographic(OrthographicProjection {
            scale: 0.01,
            ..OrthographicProjection::default_3d()
        }),
    ));
}

fn spawn_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let sphere = meshes.add(Circle::new(1.));
    let blue = materials.add(Color::srgb(0.4, 0.4, 0.6));

    let size = Vec2::new(20., 3.);
    commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::from_size(Vec2::ONE))),
            MeshMaterial2d(blue.clone()),
            Transform {
                scale: size.extend(0.),
                ..Default::default()
            },
        ))
        .insert(StaticBoxColliderBundle {
            pos: Pos(Vec2::new(0., -3.)),
            collider: BoxCollider { size },
            restitution: Restitution(0.),
            ..Default::default()
        });
    let radius = 0.15;
    let stacks = 5;
    for i in 0..15 {
        for j in 0..stacks {
            let pos = Vec2::new(
                (j as f32 - stacks as f32 / 2.) * 2.5 * radius,
                2. * radius * i as f32 - 2.,
            );
            let vel = Vec2::ZERO;

            commands
                .spawn((
                    Mesh2d(sphere.clone()),
                    MeshMaterial2d(blue.clone()),
                    Transform {
                        scale: Vec3::splat(radius),
                        translation: pos.extend(0.),
                        ..Default::default()
                    },
                ))
                .insert(ParticleBundle {
                    collider: CircleCollider { radius },
                    restitution: Restitution(0.),
                    ..ParticleBundle::new_with_pos_and_vel(pos, vel)
                });
        }
    }
}
