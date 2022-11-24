use bevy::{prelude::*, utils::FloatOrd};

use crate::*;

#[derive(Default, Reflect, Component)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_offset: Vec3,
}

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>().add_system(tower_shooting);
    }
}

fn tower_shooting(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut towers: Query<(Entity, &GlobalTransform, &mut Tower)>,
    targets: Query<&GlobalTransform, With<Target>>,

    bullet_assets: Res<GameAssets>,
    time: Res<Time>,
) {
    for (tower_ent, transform, mut tower) in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let bullet_spawn = transform.translation() + tower.bullet_offset;

            let direction = targets
                .iter()
                .min_by_key(|target_transform| {
                    FloatOrd(Vec3::distance(target_transform.translation(), bullet_spawn))
                })
                .map(|closest_target| closest_target.translation() - bullet_spawn);

            if let Some(direction) = direction {
                commands.entity(tower_ent).with_children(|commands| {
                    commands.spawn(BulletBundle {
                        bullet: Bullet {
                            direction: direction,
                            speed: 2.0,
                        },
                        scene: SceneBundle {
                            scene: bullet_assets.bullet_scene.clone(),
                            transform: Transform::from_translation(tower.bullet_offset),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
            }
        }
    }
}
