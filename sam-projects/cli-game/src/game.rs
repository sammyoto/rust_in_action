use crate::entity::Entity;
use crate::player::Player;
use crate::enemy::Enemy;
use std::io;

struct Game {
    enemy_names: Vec<String>,
    player: Player
}

impl Game {
    // pub fn new() -> Self {
    //     let enemy_names: Vec<String> = 
    //     Self {
    //         enemy_names: "",
    //         player: ""
    //     }
    // }
    fn prompt_user(&self, prompt: &str) -> String {
    println!("{}", prompt);

    let mut response: String = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read line");

    response.trim().to_string()
    }
    fn generate_enemy(&self) -> Enemy {
        Enemy{ entity: Entity{name: "bruh".to_string(), hp: 100, attack_power: 100, magic_power: 100, gold: 100}}
    }
    fn run_game(&self) {

    }
}