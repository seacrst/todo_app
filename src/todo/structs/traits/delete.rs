use crate::state::{write_to_file, StateMap};

pub trait Delete {
  fn delete(&self, title: &String, state: &mut StateMap) {
    state.remove(title);
    write_to_file("./state.json".to_string(), state);
    println!("\n\n{} is being deleted\n\n", title);
  }
}