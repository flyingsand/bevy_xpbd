use bevy::prelude::*;

use crate::*;

#[derive(Debug, Bundle, Default)]
pub struct ParticleBundle {
    pub pos: Pos,
    pub pre_pos: PrePos,
    pub mass: Mass,
    pub collider: CircleCollider,
    pub vel: Vel,
    pub pre_solve_vel: PreSolveVel,
    pub restitution: Restitution,
}

impl ParticleBundle {
    pub fn new_with_pos_and_vel(pos: Vec2, vel: Vec2) -> Self {
        Self {
            pos: Pos(pos),
            pre_pos: PrePos(pos - vel * DELTA_TIME),
            vel: Vel(vel),
            ..Default::default()
        }
    }
}

#[derive(Bundle, Default)]
pub struct StaticCircleColliderBundle {
    pub pos: Pos,
    pub collider: CircleCollider,
    pub restitution: Restitution,
}

#[derive(Bundle, Default)]
pub struct StaticBoxColliderBundle {
    pub pos: Pos,
    pub collider: BoxCollider,
    pub restitution: Restitution,
}
