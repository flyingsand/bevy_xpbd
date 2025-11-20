use bevy::prelude::*;
use bevy_xpbd::*;
use rand::random;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.8, 0.8, 0.9)))
        .insert_resource(Time::from_hz(1. / DELTA_TIME))
        .add_plugins(DefaultPlugins)
        .add_plugins(XpbdPlugins)
        .add_systems(Startup, startup)
        .add_systems(FixedUpdate, spawn_marbles)
        .add_systems(Update, despawn_marbles)
        .run();
}
#[derive(Resource)]
struct Materials {
    blue: Handle<ColorMaterial>,
}
#[derive(Resource)]
struct Meshes {
    sphere: Handle<Mesh>,
}

#[derive(Resource)]
struct SpawnTimer {
    timer: Timer,
}
fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let sphere = meshes.add(Circle::new(1.));
    let blue = materials.add(Color::srgb(0.4, 0.4, 0.6));

    commands.spawn((
        Camera2d,
        Transform::from_translation(Vec3::new(0., 0., 100.)),
        Projection::Orthographic(OrthographicProjection {
            scale: 0.01,
            ..OrthographicProjection::default_2d()
        }),
    ));
    //let radius = 15.;
    let size = Vec2::new(100., 3.);
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
            pos: Pos(Vec2::new(-54., 0.)),
            collider: BoxCollider { size },
            restitution: Restitution(0.),
            ..Default::default()
        });
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
            pos: Pos(Vec2::new(54., 0.)),
            collider: BoxCollider { size },
            restitution: Restitution(0.),
            ..Default::default()
        });
    commands.insert_resource(Meshes { sphere: sphere });
    commands.insert_resource(Materials {
        blue: materials.add(Color::srgb(0.4, 0.4, 0.6)),
    });
    commands.insert_resource(SpawnTimer {
        timer: Timer::from_seconds(0.05, TimerMode::Repeating),
    });
}

fn spawn_marbles(
    time: Res<Time>,
    mut commands: Commands,
    materials: Res<Materials>,
    meshes: Res<Meshes>,
    mut timer: ResMut<SpawnTimer>,
) {
    let dt = time.delta();
    timer.timer.tick(dt);
    if !timer.timer.just_finished() {
        return;
    }
    let radius = 0.1;
    let pos = Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5) * 0.5 + Vec2::Y * 3.;
    let vel = Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5);
    commands
        .spawn((
            Mesh2d(meshes.sphere.clone()),
            MeshMaterial2d(materials.blue.clone()),
            Transform {
                scale: Vec3::splat(radius),
                translation: pos.extend(0.),
                ..Default::default()
            },
        ))
        .insert(ParticleBundle {
            collider: CircleCollider { radius },
            restitution: Restitution(0.),
            ..ParticleBundle::new_with_pos_and_vel(pos, vel * 0.1)
        });
}

fn despawn_marbles(mut commands: Commands, query: Query<(Entity, &Pos)>) {
    for (entity, pos) in query.iter() {
        if pos.0.y < -20. {
            commands.entity(entity).despawn();
        }
    }
}
