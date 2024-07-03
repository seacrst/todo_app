use serde_json::json;

use crate::state::{write_to_file, StateMap, STATE_JSON};

pub trait Edit {
  fn set_to_done(&self, title: &String, state: &mut StateMap) {
    state.insert(title.to_string(), json!(String::from("done")));
    write_to_file(STATE_JSON.to_string(), state);
    println!("\n\n{} is being set to done\n\n", title);
  }
  fn set_to_pending(&self, title: &String, state: &mut StateMap) {
    state.insert(title.to_string(), json!(String::from("pending")));
    write_to_file(STATE_JSON.to_string(), state);
    println!("\n\n{} is being set to pending\n\n", title);
  }
}