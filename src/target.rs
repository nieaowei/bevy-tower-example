use crate::common::Health;
use crate::physics::PhysicsBundle;
use crate::GameAssets;
use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Target {
    pub speed: f32,
}

#[derive(Bundle)]
pub struct TargetBundle {
    name: Name,
    target: Target,
    health: Health,
    physics: PhysicsBundle,
    scene: SceneBundle,
}

impl Default for TargetBundle {
    fn default() -> Self {
        Self {
            scene: SceneBundle::default(),
            name: Name::new("Target"),
            target: Target { speed: 0.3 },
            health: Health { value: 3 },
            physics: PhysicsBundle::moving_entity(Vec3::new(0.168, 0.299, 0.168)),
        }
    }
}

impl From<SceneBundle> for TargetBundle {
    fn from(scene: SceneBundle) -> Self {
        Self { scene, ..default() }
    }
}

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Target>()
            .register_type::<Health>()
            .add_startup_system(setup)
            .add_system(move_targets)
            .add_system(target_death)
            .add_system(spawn_target);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(TargetSpawn::default());
}

fn move_targets(mut targets: Query<(&Target, &mut Transform)>, time: Res<Time>) {
    for (target, mut ts) in &mut targets {
        ts.translation.x += target.speed * time.delta_seconds();
    }
}

fn target_death(mut commands: Commands, targets: Query<(Entity, &Health)>) {
    for (ent, health) in &targets {
        if health.value <= 0 {
            commands.entity(ent).despawn_recursive();
        }
    }
}

#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct TargetSpawn {
    speed: f32,
    time: Timer,
    ts: Transform,
}

impl Default for TargetSpawn {
    fn default() -> Self {
        Self {
            speed: Default::default(),
            time: Timer::from_seconds(2.0, TimerMode::Repeating),
            ts: Transform::from_xyz(-4.0, 0.168, 1.5),
        }
    }
}

fn spawn_target(
    mut commands: Commands,
    mut target_spawn: Query<&mut TargetSpawn>,
    assets: Res<GameAssets>,
    time: Res<Time>,
) {
    let mut target_spawn = target_spawn.single_mut();
    target_spawn.time.tick(time.delta());
    if target_spawn.time.just_finished() {
        commands.spawn(TargetBundle::from(SceneBundle {
            scene: assets.target_scene.clone(),
            transform: target_spawn.ts,
            ..default()
        }));
    }
}
