use crate::common::*;

pub(crate) trait ErrAt<T>: Sized {
  fn err_at(self, location: impl Into<Location>) -> Result<T, Error>;
}

impl<T, E: At> ErrAt<T> for Result<T, E> {
  fn err_at(self, location: impl Into<Location>) -> Result<T, Error> {
    match self {
      Ok(ok) => Ok(ok),
      Err(err) => Err(err.at(location.into())),
    }
  }
}
