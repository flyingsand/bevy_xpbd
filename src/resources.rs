use bevy::prelude::*;

#[derive(Debug, Default, Deref, DerefMut, Resource)]
pub struct Contacts(pub Vec<(Entity, Entity, Vec2)>);

#[derive(Debug, Default, Deref, DerefMut, Resource)]
pub struct StaticContacts(pub Vec<(Entity, Entity, Vec2)>);

#[derive(Debug, Default, Deref, DerefMut, Resource)]
pub struct CollisionPairs(pub Vec<(Entity, Entity)>);

#[derive(Debug, Resource, Deref)]
pub struct Gravity(pub Vec2);
impl Default for Gravity {
    fn default() -> Self {
        Self(Vec2::new(0., -9.8))
    }
}
