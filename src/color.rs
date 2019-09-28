use crate::common::*;

use ansi_term::{ANSIGenericString, Color::Red, Style};
use atty::Stream;

#[derive(Copy, Clone)]
pub(crate) struct Color {
  atty: bool,
  style: Style,
}

impl Color {
  pub(crate) fn new() -> Color {
    Color {
      atty: false,
      style: Style::new(),
    }
  }

  pub(crate) fn stderr(self) -> Color {
    Color {
      atty: atty::is(Stream::Stderr),
      ..self
    }
  }

  pub(crate) fn error(self) -> Color {
    self.restyle(Style::new().fg(Red).bold())
  }

  pub(crate) fn message(self) -> Color {
    self.restyle(Style::new().bold())
  }

  pub(crate) fn paint<'a>(self, text: &'a str) -> ANSIGenericString<'a, str> {
    self.style().paint(text)
  }

  pub(crate) fn active(self) -> bool {
    self.atty
  }

  pub(crate) fn wrap<T: Display>(self, value: T) -> impl Display {
    Wrapped { color: self, value }
  }

  fn style(self) -> Style {
    if self.active() {
      self.style
    } else {
      Style::new()
    }
  }

  fn restyle(self, style: Style) -> Color {
    Color { style, ..self }
  }
}

struct Wrapped<T: Display> {
  color: Color,
  value: T,
}

impl<T: Display> Display for Wrapped<T> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    self.color.style().prefix().fmt(f)?;
    self.value.fmt(f)?;
    self.color.style().suffix().fmt(f)?;
    Ok(())
  }
}
