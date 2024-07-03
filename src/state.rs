use std::{
  fs::{self, File}, io::{Read, Write}
};

use serde_json::{
  json,
  Map,
  value::Value
};

pub const STATE_JSON: &str = "./todos.json";

pub fn create_file(file_name: &str) {
  File::create(file_name)
    .expect("Failed to create file")
    .write("{}".as_bytes())
    .expect("Failed to prepare file");
}

pub type StateMap = Map<String, Value>;

pub fn read_file(file_name: &str) -> StateMap {
  let mut file = match File::open(file_name) {
    Ok(file) => file,
    Err(_) => {
      create_file(file_name);
      File::open(file_name).expect("Failed to open file")
    }
  };
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

