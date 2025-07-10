mod player;
mod entity;
mod game;
mod enemy;
mod util;

use player::Player;
use entity::Entity;
use entity::EntityBehavior;
use util::read_csv;


fn main() {
  let mut player = Player{entity: Entity {name: "sam".to_string(), hp: 100, attack_power: 20, magic_power: 10, gold: 50}};
  player.print();

  let names: Vec<String> = read_csv().unwrap();
}
