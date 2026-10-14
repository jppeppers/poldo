use crate::list::item::Item;

#[derive(Clone, Default, Debug)]
pub struct List {
  pub id: usize,
  pub name: String,
  items: Option<Vec<Item>>
}

impl List {
  pub fn new(id: usize, name: &str) -> List {
    Self {
      id: id,
      name: String::from(name),
      items: None
    }
  }
}
