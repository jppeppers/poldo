// Application
use crate::todo_application::todo::Todo;

// Messages
use crate::messages::list::message::Message;

// External
use iced::widget::{button, Column, column, text};
use iced::Element;

#[derive(Default)]
pub struct NavMenu {}

impl NavMenu {
  pub fn render(app: &Todo) -> Column<'_, Message> {
    column![
      text("This is the sidebar")
    ]
  }
}
