use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum GameError
{
  UnseenEmpty,
  EmptyCommit,
  NextCalledWithUncommitedResult,
  GameOver,
}

// TODO:
//   This should give proper error messages.
impl Display for GameError
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    writeln!(f, "{:?}", self)
  }
}

impl std::error::Error for GameError {}
