// Application
use crate::Todo;

// Messages
use crate::application::Message;

// Views
use crate::application::views::{ActionsMenu, NavMenu};

// External
use iced::widget::{Column, button, column, container, row, text};
use iced::{Element, Length::Fill};

#[derive(Default)]
pub struct ListView {}

impl ListView {
  pub fn render(app: &Todo) -> Element<'_, Message> {
    let actions_menu = ActionsMenu::render();
    let sidebar = NavMenu::render(app);

    let view_text =
      if app.selected_list.is_some() {
        text(app.selected_list.clone().unwrap().name.clone())
      } else {
        text("No Selected List")
      };

    let view: Column<'_, Message> = column![
      view_text
    ];

    column![
      actions_menu,
      row![
        column![sidebar].height(Fill),
        column![view].height(Fill)
      ].height(Fill)
    ].into()
  }
}

#[derive(Default)]
pub struct NewListView {}

impl NewListView {
  pub fn render(app: &Todo) -> Element<'_, Message> {
    let sidebar: container::Container<'_, Message> = NavMenu::render(app);

    let view_column: Column<'_, Message> = column![
      text("You are viewing the NEW list form"),
      // button("SWITCH VIEWS").on_press(Message::NavigateToListView)
    ];

    let view = container(view_column);

    row![
      column![sidebar].height(Fill),
      column![view].height(Fill)
    ].height(Fill).into()
  }
}
