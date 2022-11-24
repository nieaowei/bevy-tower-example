use bevy::prelude::*;
use crate::common::Lifetime;

use crate::physics::{self, PhysicsBundle};

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Bullet {
    pub direction: Vec3,
    pub speed: f32,
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Bullet>()
            .register_type::<Lifetime>()
            .add_system(move_bullets)
            .add_system(bullet_despawn);
    }
}


fn move_bullets(mut bullets: Query<(&Bullet, &mut Transform)>, time: Res<Time>) {
    for (bullet, mut ts) in &mut bullets {
        ts.translation += bullet.direction.normalize() * bullet.speed * time.delta_seconds();
    }
}

fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (e, mut lf) in &mut bullets {
        lf.timer.tick(time.delta());
        if lf.timer.just_finished() {
            commands.entity(e).despawn_recursive();
        }
    }
}

#[derive(Bundle)]
pub struct BulletBundle {
    pub bullet: Bullet,
    pub lifetime: Lifetime,
    pub physics: PhysicsBundle,
    pub scene: SceneBundle,
}

impl Default for BulletBundle {
    fn default() -> Self {
        Self {
            scene: Default::default(),
            bullet: Default::default(),
            lifetime: Lifetime {
                timer: Timer::from_seconds(1000.5, TimerMode::Once),
            },
            physics: physics::PhysicsBundle::moving_entity_ball(0.05),
        }
    }
}
