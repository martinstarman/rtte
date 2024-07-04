use crate::{
  component::{
    melee_attack::MeleeAttackComponent, player::PlayerComponent, selection::SelectionComponent,
  },
  event::knife_melee_attack::KnifeMeleeAttack,
  resource::cursor::{Cursor, CursorType},
};
use bevy_ecs::{
  event::EventReader,
  query::With,
  system::{Query, ResMut},
};

// TODO: melee attack
pub fn knife_melee_attack(
  mut events: EventReader<KnifeMeleeAttack>,
  mut query: Query<(&SelectionComponent, &mut MeleeAttackComponent), With<PlayerComponent>>,
  mut cursor: ResMut<Cursor>,
) {
  for _event in events.read() {
    for (selection, mut melee_attack) in &mut query {
      // TODO: can player have melee attack?
      if selection.active {
        melee_attack.active = true;
        cursor.set_type(CursorType::Knife);
      }
    }
  }
}
