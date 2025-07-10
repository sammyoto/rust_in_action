#[derive(Debug)]
pub struct Entity {
  pub name: String,
  pub hp: u16,
  pub current_hp: u16,
  pub attack_power: u16,
  pub magic_power: u16,
  pub gold: u16
}

pub trait EntityBehavior {
  fn entity(&self) -> &Entity;
  fn entity_mut(&mut self) -> &mut Entity;
  // Print player
  fn print(&self) {
    let entity: &Entity = self.entity();

    println!("\n======================");
    println!("👤 Name        : {}", entity.name);
    println!("❤️ Health      : {}/{}", entity.current_hp,entity.hp);
    println!("⚔️  Attack Power: {}", entity.attack_power);
    println!("🪄 Magic Power : {}", entity.magic_power);
    println!("💰 Gold        : {}", entity.gold);
    println!("======================\n");
  }
  // Getters and setters
  fn get_name(&self) -> String {
    self.entity().name.clone()
  }
  fn get_hp(&self) -> u16 {
    self.entity().hp
  }
  fn get_current_hp(&self) -> u16 {
    self.entity().current_hp
  }
  fn get_attack_power(&self) -> u16 {
    self.entity().attack_power
  }
  fn get_magic_power(&self) -> u16 {
    self.entity().magic_power
  }
  fn get_gold(&self) -> u16 {
    self.entity().gold
  }
  fn set_name(&mut self, name: String) {
    self.entity_mut().name = name
  }
  fn set_hp(&mut self, hp: u16) {
    self.entity_mut().hp = hp
  }
  fn set_current_hp(&mut self, current_hp: u16) {
    self.entity_mut().current_hp = current_hp
  }
  fn set_attack_power(&mut self, attack_power: u16) {
    self.entity_mut().attack_power = attack_power
  }
  fn set_magic_power(&mut self, magic_power: u16) {
    self.entity_mut().magic_power = magic_power
  }
  fn set_gold(&mut self, gold: u16) {
    self.entity_mut().gold = gold
  }
}

impl EntityBehavior for Entity {
  fn entity(&self) -> &Entity {
    self
  }
  fn entity_mut(&mut self) -> &mut Entity {
    self
  }
}