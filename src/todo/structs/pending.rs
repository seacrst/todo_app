use super::{base::Base, traits::{create::Create, delete::Delete, edit::Edit, get::Get}};

pub struct Pending {
  pub base: Base
}

impl Pending {
    pub fn new(title: &str) -> Self {
      let base = Base::new(title, "pending");
      Self {base}
    }
}

impl Get for Pending {}
impl Create for Pending {}
impl Edit for Pending {}
impl Delete for Pending {}