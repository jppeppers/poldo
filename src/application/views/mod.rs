pub mod application;
pub mod settings;

pub use application::ActionsMenu;
pub use application::NavMenu;
pub use settings::SettingsView;

#[derive(Default)]
pub enum View {
  // Default - view a list
  #[default]
  List,
  // Form for creating a new list
  NewList,
  // Settings modal
  Settings,
}
