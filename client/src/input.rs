use std::io;

pub fn get_wallet() -> String {
  let mut address = String::new();
  io::stdin().read_line(&mut address).unwrap();
  address
}

pub fn get_request_type() -> i32 {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  let decision: i32 = input.trim().parse().unwrap();
  decision
}

pub fn get_final_input() -> String {
  let mut final_input = String::new();
  io::stdin().read_line(&mut final_input).unwrap();
  let result: String = final_input.trim().into();
  result
}

pub fn get_display(decision: i32) -> String {
  let mut display = String::new();
    match decision {
        1 => display = "Full".into(),
        2 => display = "Spent".into(),
        3 => display = "Unspent".into(),
        _ => println!("Input error")
    }
  display
}