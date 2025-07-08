mod player;
mod entity;

use std::io;
use player::Player;
use entity::Entity;
use entity::EntityBehavior;

fn prompt_user(prompt: &str) -> String {
  // Prompt user
  println!("{}", prompt);

  let mut response: String = String::new();
  io::stdin().read_line(&mut response).expect("Failed to read line");

  response.trim().to_string()
}

fn main() {
  let mut player = Player{entity: Entity {name: "sam".to_string(), hp: 100, attack_power: 20, magic_power: 10, gold: 50}};
  player.print();
}
