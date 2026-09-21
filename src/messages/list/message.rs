#[derive(Debug, Clone)]
pub enum Message {
  // Navigate to view containing a specific list
  NavigateToListView,
  // Navigate to form to add a new list
  NavigateToNewListView,
}
