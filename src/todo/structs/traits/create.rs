use serde_json::json;

use crate::state::{write_to_file, StateMap, STATE_JSON};

pub trait Create {
  fn create(&self, title: &String, status: &String, state: &mut StateMap) {
    state.insert(title.to_string(), json!(status));
    write_to_file(STATE_JSON.to_string(), state);
    println!("\n\n{} is being created\n\n", title);
  }
}