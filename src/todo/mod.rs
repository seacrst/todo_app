pub mod structs;

use structs::{
  pending::Pending, 
  done::Done
};

pub enum Status {
  Done(Done),
  Pending(Pending)
}

pub fn factory(status: &str, title: &str) -> Result<Status, &'static str> {
  match status {
    "pending" => Ok(Status::Pending(Pending::new(title))),
    "done" => Ok(Status::Done(Done::new(title))),
    _ => Err("This is no accepted")
  }
}