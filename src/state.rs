use std::{
  io::Read,
  fs::{self, File}
};

use serde_json::{
  json,
  Map,
  value::Value
};


pub type StateMap = Map<String, Value>;

pub fn read_file(file_name: &str) -> StateMap {
  let mut file = File::open(file_name).expect("Failed to read file");
  let mut data = String::new();

  file.read_to_string(&mut data).unwrap();
  let json: Value = serde_json::from_str(&data).unwrap();
  let state = json.as_object().unwrap().clone();

  state
}

pub fn write_to_file(file_name: String, state: &mut StateMap) {
  let contents = json!(state).to_string();
  fs::write(file_name, contents).expect("Unable to write file");
} 

