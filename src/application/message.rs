use crate::list::List;

#[derive(Clone)]
pub enum Message {
  // Open the "New List" form
  OpenNewListForm,
  // Set selected list
  SetSelectedList(List),
}
