mod application;
mod list;
mod test;

use list::{List, views::ListView};
use application::Message;

use iced::{self, color, Element, Theme};
use iced::theme::Palette;

fn main() -> iced::Result {
  iced::application(
    Todo::new,
    Todo::update,
    Todo::view,
  ).theme(Todo::theme).run()
}

#[derive(Default)]
pub struct Todo {
  pub selected_list: Option<List>,
  pub lists: Vec<List>,
}

impl Todo {
  pub fn new() -> Self {
    Self {
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

  pub fn theme(&self) -> Theme {
    Theme::custom("Panda", Self::panda_palette())
  }

  pub fn panda_palette() -> Palette {
    Palette {
      background: color!(0x2A2C2D),
      text:       color!(0xE6E6E6),
      primary:    color!(0x45A9F9),
      success:    color!(0x6FE7D2),
      warning:    color!(0xFFCC95),
      danger:     color!(0xFF4B82),
    }
  }
}
