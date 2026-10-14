mod application;
mod list;

use list::{List, views::ListView};
use application::Message;

use iced::{self, Element};

fn main() -> iced::Result {
  iced::application(
    Todo::new,
    Todo::update,
    Todo::view,
  ).run()
}

#[derive(Default)]
pub struct Todo {
  pub selected_list: Option<List>,
  pub lists: Vec<List>,
}

impl Todo {
  pub fn new() -> Self {
    Todo {
      selected_list: None,
      lists: vec![
        List::new(0, "List 1"),
        List::new(1, "List 2"),
        List::new(2, "List 3"),
      ],
    }
  }

  pub fn update(&mut self, message: Message) {
    match message {
      Message::OpenNewListForm => {
        println!("Open New Form!")
      }
      Message::SetSelectedList(list) => {
        self.selected_list = Some(list)
      }
    }
  }

  pub fn view(&self) -> Element<'_, Message> {
    ListView::render(self)
  }
}
