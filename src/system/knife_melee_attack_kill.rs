use crate::component::{
  enemy::{EnemyComponent, EnemyState},
  melee_attack::MeleeAttackComponent,
  player::PlayerComponent,
  position::PositionComponent,
};
use bevy_ecs::{query::With, system::Query};
use maths_rs::{distance, vec::Vec2};

// TODO: melle attack kill
pub fn knife_melee_attack_kill(
  query1: Query<(&PositionComponent, &MeleeAttackComponent), With<PlayerComponent>>,
  mut query2: Query<(&mut EnemyComponent, &PositionComponent)>,
) {
  for (player_position, melee_attack) in &query1 {
    if let Some(enemy_id) = melee_attack.enemy_id {
      println!("{:?}", enemy_id);
      for (mut enemy, enemy_position) in &mut query2 {
        if melee_attack.active && enemy_id == enemy.id {
          println!("A");

          let distance = distance(
            Vec2::new(player_position.x, player_position.y),
            Vec2::new(enemy_position.x, enemy_position.y),
          );

          println!("dist: {}", distance);

          // TODO: constants
          if distance < 20. {
            enemy.state = EnemyState::DEAD;
            println!("DEAD");
          }
        }
      }
    }
  }
}
