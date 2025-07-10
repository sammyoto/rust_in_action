use crate::entity::{Entity, EntityBehavior};
use crate::player::Player;
use crate::enemy::Enemy;
use crate::util::read_csv;
use std::io;


pub struct Game {
    enemy_names: Vec<String>,
    player: Player
}

impl Game {
    pub fn new() -> Self {
        Self {
            enemy_names: read_csv().unwrap(),
            player: Player{entity: Entity{name: "".to_string(), hp: 100, attack_power: 20, magic_power: 10, gold: 50}}
        }
    }
    fn prompt_user(&self, prompt: &str) -> String {
    println!("{}", prompt);

    let mut response: String = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read line");

    response.trim().to_string()
    }
    fn generate_enemy(&self) -> Enemy {
        Enemy{ entity: Entity{name: "bruh".to_string(), hp: 100, attack_power: 100, magic_power: 100, gold: 100}}
    }
    pub fn run_game(&mut self) {
        let prompt: &str = "Name your character!";
        self.player.set_name(self.prompt_user(prompt));
        self.player.print();
    }
}