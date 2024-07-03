use super::{base::Base, traits::{delete::Delete, edit::Edit, get::Get}};

pub struct Done {
  pub base: Base
}

impl Done {
    pub fn new(title: &str) -> Self {
      let base = Base::new(title, "done");
      Self {base}
    }
}


impl Get for Done {}
impl Edit for Done {}
impl Delete for Done {}