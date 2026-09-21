// Application
use crate::todo_application::todo::Todo;

// Messages
use crate::messages::list::message::Message;

// Views
use crate::views::application::application::NavMenu;

// External
use iced::widget::{button, column, Grid, pane_grid, text};
use iced::{Element};

#[derive(Default)]
pub struct ListView {}

impl ListView {
  pub fn render(app: &Todo) -> Element<'_, Message> {
    let sidebar = NavMenu::render(app);

    let view = column![
      text("You are viewing the list view"),
      button("SWITCH VIEWS").on_press(Message::NavigateToNewListView)
    ];

    pane_grid(app, {

    }).into()
  }
}

#[derive(Default)]
pub struct NewListView {}

impl NewListView {
  pub fn render(app: &Todo) -> Element<'_, Message> {
    let sidebar = NavMenu::render(app);

    let view = column![
      text("You are viewing the NEW list form"),
      button("SWITCH VIEWS").on_press(Message::NavigateToListView)
    ];

    Grid::with_children([
      sidebar.into(),
      view.into(),
    ]).into()
  }
}
