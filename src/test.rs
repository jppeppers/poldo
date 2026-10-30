// Derived from: https://github.com/tinkertrain/panda-syntax-vscode/blob/master/themes/colors.yaml

use iced::{self, color, Theme};
use iced::theme::Palette;

#[derive(Default)]
pub struct State {}

impl State {
  fn theme(&self) -> Theme {
    Theme::custom("Panda Syntax", Self::panda_palette())
  }

  fn panda_palette() -> Palette {
     Palette {
      background: color!(0x2A2C30),
      text:       color!(0xE5E5E5),
      primary:    color!(0x7DC1FF),
      success:    color!(0x35FFDC),
      warning:    color!(0xFFB86C),
      danger:     color!(0xEC2864),
    }
  }
}
