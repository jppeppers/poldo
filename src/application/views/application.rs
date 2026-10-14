// Application
use crate::Todo;

// Messages
use crate::application::Message;

// External
use iced::widget::{
  Container, Column, button, container, row, text
};
use iced::Length::Fill;

#[derive(Default)]
pub struct NavMenu {}

impl NavMenu {
  pub fn render(app: &Todo) -> Container<'_, Message> {
    let sidebar_column = Column::with_children(
      app.lists.iter().map( |l| 
        row![
          button(text(l.name.clone()))
            .on_press(Message::SetSelectedList(l.clone()))
            .width(Fill)
        ].into()
      )
    );

    container(sidebar_column)
      .style(container::bordered_box)
      .width(iced::Length::Fixed(250.0))
      .height(Fill)
  }
}

#[derive(Default)]
pub struct ActionsMenu {}

impl ActionsMenu {
  pub fn render() -> Container<'static, Message> {
    container(row![
      button("+ New Todo List")
        .on_press(Message::OpenNewListForm)
    ])
      .style(container::bordered_box)
      .align_right(Fill)
      .padding(iced::Padding::new(10.0).right(10))
  }
}
