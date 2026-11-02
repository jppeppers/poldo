use crate::Todo;
use crate::application::Message;
use crate::list::views::ListView;

use iced::{self, Element};
use iced::widget::{container, stack, opaque, text};

pub struct SettingsView {}

impl SettingsView {
  pub fn render(app: &Todo) -> Element<'_, Message> {
    container(
      stack![
        ListView::render(&app),
        opaque(
          text("Tehe")
        )
      ]
    ).into()
  }
}
