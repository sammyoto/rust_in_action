mod player;
mod entity;
mod game;
mod enemy;
mod util;
mod scripts;

use player::Player;
use entity::Entity;
use entity::EntityBehavior;
use game::Game;
use util::read_csv;


fn main() {
  let mut game: Game = Game::new();
  game.run_game();
}
