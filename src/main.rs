mod todo;
mod state;
mod processes;

use std::env;
use processes::process_input;
use state::{read_file, STATE_JSON};
use todo::factory;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() == 1 {
    println!("Please enter command such as `create`, `edit`, `get`, `delete`");
    return;
  }
  
  if args.len() == 2 {
    println!("Please add item");
    return;  
  }

  let command = &args[1];
  let title = &args[2];

  let state = read_file(STATE_JSON);

  let status: String;

  match &state.get(*&title) {
      Some(result) => {
        status = result.to_string().replace('\"', "");
      },
      None => {
        status = "pending".to_string();
      }
  }

  let item = factory(&status, title).expect(&status);
  process_input(item, command.to_string(), &state)
}
