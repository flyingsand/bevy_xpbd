use bevy::prelude::*;

mod component;
mod entity;

pub use component::*;
pub use entity::*;

pub const DELTA_TIME: f64 = 1. / 240.;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum XpbdSystems {
    CollectCollisionPairs,
    Integrate,
    SolvePos,
    UpdateVel,
    SolveVel,
    SyncTransfrom,
}

pub struct XpbdPlugins;
impl Plugin for XpbdPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            collect_collision_pairs.in_set(XpbdSystems::CollectCollisionPairs),
        )
        .add_systems(
            FixedUpdate,
            (integrate, clear_contacts)
                .chain()
                .in_set(XpbdSystems::Integrate),
        )
        .add_systems(
            FixedUpdate,
            (solve_pos, solve_pos_circle_statics, solve_pos_box_statics)
                .in_set(XpbdSystems::SolvePos),
        )
        .add_systems(FixedUpdate, update_vel.in_set(XpbdSystems::UpdateVel))
        .add_systems(
            FixedUpdate,
            (solve_vel, solve_vel_statics).in_set(XpbdSystems::SolveVel),
        )
        .add_systems(
            FixedUpdate,
            sync_transfrom.in_set(XpbdSystems::SyncTransfrom),
        );
        app.configure_sets(
            FixedUpdate,
            (
                XpbdSystems::CollectCollisionPairs,
                XpbdSystems::Integrate,
                XpbdSystems::SolvePos,
                XpbdSystems::UpdateVel,
                XpbdSystems::SolveVel,
                XpbdSystems::SyncTransfrom,
            )
                .chain(),
        );
        app.init_resource::<Gravity>();
        app.init_resource::<Contacts>();
        app.init_resource::<StaticContacts>();
    }
}
#[derive(Debug, Resource, Deref)]
pub struct Gravity(pub Vec2);
impl Default for Gravity {
    fn default() -> Self {
        Self(Vec2::new(0., -9.8))
    }
}
//old simulate
/*
fn simulate(
    mut query: Query<(&mut Pos, &mut PrePos, &Mass)>,
    time: Res<Time>,
    gravity: Res<Gravity>,
) {
    let delta_time = time.delta_secs();
    for (mut pos, mut pre_pos, mass) in query.iter_mut() {
        let g_force = **mass * **gravity;
        let ex_force = g_force;
        let velocity = (pos.0 - pre_pos.0) / delta_time + delta_time * ex_force / **mass;
        pre_pos.0 = pos.0;
        pos.0 = pre_pos.0 + velocity * delta_time;
    }
}
*/
fn collect_collision_pairs() {}

fn integrate(
    mut query: Query<(&mut Pos, &mut PrePos, &mut Vel, &Mass, &mut PreSolveVel)>,
    time: Res<Time>,
    gravity: Res<Gravity>,
) {
    let delta_time = time.delta_secs();
    for (mut pos, mut pre_pos, mut vel, mass, mut pre_solve_vel) in query.iter_mut() {
        let g_force = **mass * **gravity;
        let ex_force = g_force;
        vel.0 += delta_time * ex_force / **mass;
        pre_pos.0 = pos.0;
        pos.0 += **vel * delta_time;
        pre_solve_vel.0 = **vel;
    }
}
fn clear_contacts(mut contacts: ResMut<Contacts>, mut static_contacts: ResMut<StaticContacts>) {
    contacts.0.clear();
    static_contacts.0.clear();
}
fn solve_pos(
    mut query: Query<(Entity, &mut Pos, &CircleCollider, &Mass)>,
    mut contacts: ResMut<Contacts>,
) {
    let mut iter = query.iter_combinations_mut();
    while let Some(
        [
            (entity_a, mut pos_a, circle_a, m_a),
            (entity_b, mut pos_b, circle_b, m_b),
        ],
    ) = iter.fetch_next()
    {
        let ab = pos_b.0 - pos_a.0;
        let combined_radius = circle_a.radius + circle_b.radius;
        if ab.length_squared() < combined_radius * combined_radius {
            let penetration_depth = combined_radius - ab.length();
            let n = ab.normalize();
            let w_a = 1. / **m_a;
            let w_b = 1. / **m_b;
            contacts.0.push((entity_a, entity_b, n));
            pos_a.0 -= n * penetration_depth * (w_a / (w_a + w_b));
            pos_b.0 += n * penetration_depth * (w_b / (w_a + w_b));
        }
    }
}

fn solve_pos_circle_statics(
    mut dynamics: Query<(Entity, &mut Pos, &CircleCollider), With<Mass>>,
    statics: Query<(Entity, &Pos, &CircleCollider), Without<Mass>>,
    mut contacts: ResMut<StaticContacts>,
) {
    for (entity_a, mut pos_a, col_a) in dynamics.iter_mut() {
        for (entity_b, pos_b, col_b) in statics.iter() {
            let ab = pos_b.0 - pos_a.0;
            let combined_radius = col_a.radius + col_b.radius;
            if ab.length_squared() < combined_radius * combined_radius {
                let penetration_depth = combined_radius - ab.length();
                let n = ab.normalize();
                contacts.0.push((entity_a, entity_b, n));
                pos_a.0 -= n * penetration_depth;
            }
        }
    }
}
fn solve_pos_box_statics(
    mut dynamics: Query<(Entity, &mut Pos, &CircleCollider), With<Mass>>,
    statics: Query<(Entity, &Pos, &BoxCollider), Without<Mass>>,
    mut contacts: ResMut<StaticContacts>,
) {
    for (entity_a, mut pos_a, col_a) in dynamics.iter_mut() {
        for (entity_b, pos_b, box_b) in statics.iter() {
            let ab = pos_a.0 - pos_b.0;
            let box_to_circle_abs = ab.abs();
            let half_box = box_b.size / 2.;
            let cornor_to_center = box_to_circle_abs - half_box;
            let r = col_a.radius;
            if cornor_to_center.x > r || cornor_to_center.y > r {
                continue;
            }
            let s = ab.signum();
            let (n, penetration_depth) = if cornor_to_center.x > 0. && cornor_to_center.y > 0. {
                let cornor_to_center_sqr = cornor_to_center.length_squared();
                if cornor_to_center_sqr > r * r {
                    continue;
                }
                let cornor_dist = cornor_to_center_sqr.sqrt();
                let penetration_depth = r - cornor_dist;
                let n = cornor_to_center / cornor_dist * -s;
                (n, penetration_depth)
            } else if cornor_to_center.x > cornor_to_center.y {
                (Vec2::X * -s.x, -cornor_to_center.x + r)
            } else {
                (Vec2::Y * -s.y, -cornor_to_center.y + r)
            };
            pos_a.0 -= n * penetration_depth;
            contacts.push((entity_a, entity_b, n));
        }
    }
}

fn update_vel(mut query: Query<(&Pos, &PrePos, &mut Vel)>, time: Res<Time>) {
    let dt = time.delta_secs();
    for (pos, prepos, mut vel) in query.iter_mut() {
        vel.0 = (pos.0 - prepos.0) / dt;
    }
}
fn solve_vel(query: Query<(&mut Vel, &PreSolveVel, &Mass, &Restitution)>, contacts: Res<Contacts>) {
    for (entity_a, entity_b, n) in contacts.0.iter().cloned() {
        let (
            (mut vel_a, pre_solve_vel_a, mass_a, restitution_a),
            (mut vel_b, pre_solve_vel_b, mass_b, restitution_b),
        ) = unsafe {
            // Ensure safety
            assert!(entity_a != entity_b);
            (
                query.get_unchecked(entity_a).unwrap(),
                query.get_unchecked(entity_b).unwrap(),
            )
        };
        // TODO: make sure velocities are reflected and restitution/friction calculated
        let pre_solve_relative_vel = **pre_solve_vel_a - **pre_solve_vel_b;
        let pre_solve_normal_vel = Vec2::dot(pre_solve_relative_vel, n);

        let relative_vel = **vel_a - **vel_b;
        let normal_vel = Vec2::dot(relative_vel, n);
        let restitution = (**restitution_a + **restitution_b) / 2.;

        let w_a = 1. / **mass_a;
        let w_b = 1. / **mass_b;
        let w_sum = w_a + w_b;
        let restitution_vel = (-restitution * pre_solve_normal_vel).min(0.);
        vel_a.0 += n * (-normal_vel + restitution_vel) * w_a / w_sum;
        vel_b.0 += n * (-normal_vel + restitution_vel) * w_b / w_sum;
    }
}

fn solve_vel_statics(
    mut dynamics: Query<(&mut Vel, &PreSolveVel, &Restitution), With<Mass>>,
    statics: Query<&Restitution, Without<Mass>>,
    contacts: Res<StaticContacts>,
) {
    for (entity_a, entity_b, n) in contacts.iter().cloned() {
        let (mut vel_a, pre_vel_a, restitution_a) = dynamics
            .get_mut(entity_a)
            .expect("Not get entity in query of dynamics");
        let restitution_b = statics
            .get(entity_b)
            .expect("Not get entity in query of statics");
        let pre_normal_vel = pre_vel_a.dot(n);
        let normal_vel = vel_a.0.dot(n);
        let restitution = (**restitution_a + **restitution_b) / 2.;
        vel_a.0 += n * ((-pre_normal_vel * restitution).min(0.) - normal_vel);
    }
}
fn sync_transfrom(mut query: Query<(&mut Transform, &Pos)>) {
    for (mut trans, pos) in query.iter_mut() {
        trans.translation = pos.0.extend(0.);
    }
}
