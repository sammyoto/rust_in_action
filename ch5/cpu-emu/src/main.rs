struct CPU {
  current_operation: u16,
  registers: [u8;2]
}

fn main() {
  let mut cpu = CPU {
    current_operation: 0,
    registers: [0; 2]
  }

  
}