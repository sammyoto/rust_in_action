#[derive(Debug)]
pub struct Player {
  pub name: String,
  pub hp: u16,
  pub attack_power: u16,
  pub magic_power: u16,
  pub gold: u16
}
 
impl Player {
  // Print player
  pub fn print(&self) {
    println!("{:?}", self)
  }
  // Getters and setters
  pub fn get_name(&self) -> String {
    self.name.clone()
  }
  pub fn get_hp(&self) -> u16 {
    self.hp
  }
  pub fn get_attack_power(&self) -> u16 {
    self.attack_power
  }
  pub fn get_magic_power(&self) -> u16 {
    self.magic_power
  }
  pub fn get_gold(&self) -> u16 {
    self.gold
  }
  pub fn set_name(&mut self, name: String) {
    self.name = name
  }
  pub fn set_hp(&mut self, hp: u16) {
    self.hp = hp
  }
  pub fn set_attack_power(&mut self, attack_power: u16) {
    self.attack_power = attack_power
  }
  pub fn set_magic_power(&mut self, magic_power: u16) {
    self.magic_power = magic_power
  }
  pub fn set_gold(&mut self, gold: u16) {
    self.gold = gold
  }
}