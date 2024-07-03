use crate::state::StateMap;

pub trait Get {
  fn get(&self, title: &String, state: &StateMap) {
    let item = state.get(title);

    match item {
      Some(value) => {
        println!("\n\nItem: {}", title);
        println!("Status: {}\n\n", value);
      },
      None => println!("Item {} was not found", title)
    }
  }
}