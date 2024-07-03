use crate::state::{write_to_file, StateMap, STATE_JSON};

pub trait Delete {
  fn delete(&self, title: &String, state: &mut StateMap) {
    state.remove(title);
    write_to_file(STATE_JSON.to_string(), state);
    println!("\n\n{} is being deleted\n\n", title);
  }
}