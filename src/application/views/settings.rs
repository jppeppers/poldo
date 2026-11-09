use crate::Todo;
use crate::application::Message;
use crate::list::views::ListView;

use iced::{self, Element};
use iced::widget::{container, column, stack, mouse_area, opaque, text};

pub struct SettingsView {}

impl SettingsView {
  pub fn render(app: &Todo) -> Element<'_, Message> {
    container(
      stack![
        ListView::render(&app),
        opaque(
          mouse_area(
            container(
              container(
                column![
                  text("Tehe")
                ]
              ).style(container::bordered_box)
               .width(iced::Length::Fill)
               .height(iced::Length::Fill)
            )
              .center_x(iced::Length::Fill)
              .center_y(iced::Length::Fill)
              .padding(iced::Padding::from(100.0))
          ).on_press(Message::CloseSettings)
        )
      ]
    ).into()
  }
}
