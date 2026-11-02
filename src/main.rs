mod application;
mod list;
mod test;

use list::{List, views::ListView};
use application::{Message, views::View, views::SettingsView};

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
  pub view: View,
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
      view: View::List,
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
      Message::OpenSettings => {
        self.view = View::Settings
      }
    }
  }

  pub fn view(&self) -> Element<'_, Message> {
    match self.view {
      View::List => {
        ListView::render(self)
      }
      View::NewList => {
        println!("New List Form!");
        ListView::render(self)
      }
      View::Settings => {
        SettingsView::render(self)
      }
    }
  }

  pub fn theme(&self) -> Theme {
    Theme::custom("Panda", Self::panda_palette())
  }

  pub fn panda_palette() -> Palette {
    Palette {
      background: color!(0x242526),
      text:       color!(0xE6E6E6),
      primary:    color!(0x45A9F9),
      success:    color!(0x19F9D8),
      warning:    color!(0xFFB86C),
      danger:     color!(0xFF2C6D),
    }
  }
}
