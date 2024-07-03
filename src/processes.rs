use crate::{
  state::StateMap, 
  todo::{
    Status,
    structs::{
      done::Done, 
      traits::{create::Create, delete::Delete, edit::Edit, get::Get}
    }
  }
};

use super::todo::structs::pending::Pending;

fn processe_pending(item: Pending, command: String, state: &StateMap) {
  let mut state = state.clone();

  match command.as_str() {
    "get" => item.get(&item.base.title, &state),
    "create" => item.create(&item.base.title, &item.base.status, &mut state),
    "edit" => item.set_to_done(&item.base.title, &mut state),
    "delete" => item.delete(&item.base.title, &mut state),
    _ => println!("Command {} not supported", command)
  }
}

fn process_done(item: Done, command: String, state: &StateMap) {
  let mut state = state.clone();

  match command.as_str() {
    "get" => item.get(&item.base.title, &state),
    "edit" => item.set_to_pending(&item.base.title, &mut state),
    "delete" => item.delete(&item.base.title, &mut state),
    _ => println!("Command {} not supported", command)
  }
}

pub fn process_input(status: Status, command: String, state: &StateMap) {
  match status {
      Status::Pending(item) => processe_pending(item, command, state),
      Status::Done(item) => process_done(item, command, state)
  }
}