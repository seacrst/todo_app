use serde_json::json;

use crate::state::{write_to_file, StateMap};

pub trait Create {
  fn create(&self, title: &String, status: &String, state: &mut StateMap) {
    state.insert(title.to_string(), json!(status));
    write_to_file("./state.json".to_string(), state);
    println!("\n\n{} is being created\n\n", title);
  }
}