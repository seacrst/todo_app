mod todo;
mod state;
mod processes;

use std::env;
use processes::process_input;
use state::read_file;
use todo::factory;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() == 1 {
    println!("Please enter command such as `create`, `edit`, `get`, `delete`");
    return;
  }
  
  if args.len() == 2 {
    println!("Please add task to store");
    return;  
  }

  let command = &args[1];
  let title = &args[2];

  let state_json = String::from("./state.json");

  let state = read_file(&state_json);

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
