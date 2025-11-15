use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Pos(pub Vec2);

#[derive(Component, Debug, Default)]
pub struct PrePos(pub Vec2);

#[derive(Component, Debug, Deref)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Self(1.)
    }
}

#[derive(Component, Debug)]
pub struct CircleCollider {
    pub radius: f32,
}

impl Default for CircleCollider {
    fn default() -> Self {
        Self { radius: 50. }
    }
}
#[derive(Component, Debug, Default, Deref)]
pub struct Vel(pub Vec2);

#[derive(Component, Debug, Default, Deref)]
pub struct PreSolveVel(pub Vec2);

#[derive(Debug, Default, Deref, DerefMut, Resource)]
pub struct Contacts(pub Vec<(Entity, Entity, Vec2)>);

#[derive(Debug, Default, Deref, DerefMut, Resource)]
pub struct StaticContacts(pub Vec<(Entity, Entity, Vec2)>);

#[derive(Component, Debug, Deref)]
pub struct Restitution(pub f32);

impl Default for Restitution {
    fn default() -> Self {
        Self(0.3)
    }
}
