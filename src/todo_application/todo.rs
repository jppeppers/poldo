// Messages
use crate::messages::list::message::Message;

// Models
use crate::todo_application::list::list::List;

// Views
use crate::views::list::list::{ListView, NewListView};

// External
use iced::Element;

#[derive(Default, Debug, Clone)]
enum TodoViewOption {
  #[default]
  List,
  NewList,
}

#[derive(Default)]
pub struct Todo {
  current_view: TodoViewOption,
  lists: Vec<List>,
}

impl Todo {
  pub fn new() -> Self {
    Todo {
      current_view: TodoViewOption::List,
      lists: Vec::new(),
    }
  }

  pub fn update(&mut self, message: Message) {
    match message {
      Message::NavigateToNewListView => {
        self.current_view = TodoViewOption::NewList
      },
      Message::NavigateToListView => {
        self.current_view = TodoViewOption::List
      },
    }
  }

  pub fn view(&self) -> Element<'_, Message> {
    match self.current_view {
      TodoViewOption::List => ListView::render(self),
      TodoViewOption::NewList => NewListView::render(self)
    }
  }
}