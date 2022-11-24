use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Health {
    pub value: i32,
}

#[derive(Default, Reflect, Component)]
#[reflect(Component)]
pub struct Lifetime {
    pub timer: Timer,
}