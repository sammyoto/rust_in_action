use std::io;
mod player;
use player::Player;

fn prompt_user(prompt: &str) -> String {
  // Prompt user
  println!("{}", prompt);

  let mut response: String = String::new();
  io::stdin().read_line(&mut response).expect("Failed to read line");

  response.trim().to_string()
}

fn main() {
  let mut player = Player {name: "".to_string(), hp: 100, attack_power: 20, magic_power: 10, gold: 50};
  let response: String = prompt_user("Name your character!");
  player.set_name(response);

  println!("{} sets out on their first journey!", player.get_name());
  println!("{}'s stats:", player.get_name());
  player.print();
}
