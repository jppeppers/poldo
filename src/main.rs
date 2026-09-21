use iced::application;

mod messages;
mod todo_application;
mod views;

use todo_application::todo::Todo;

fn main() -> iced::Result {
  application(
    Todo::new,
    Todo::update,
    Todo::view,
  ).run()
}
