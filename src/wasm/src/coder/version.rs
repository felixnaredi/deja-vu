use std::{
  error::Error,
  fmt::Display,
  str::FromStr,
};

pub enum GameOverCoderVersion
{
  Version00Coding,
  GameOverCoderV01,
}

impl TryFrom<&String> for GameOverCoderVersion
{
  type Error = GameOverCoderVersionError;

  fn try_from(s: &String) -> Result<Self, Self::Error>
  {
    use GameOverCoderVersion::*;

    match s.as_str() {
      "00" => Ok(Version00Coding),
      "goc-v01" => Ok(GameOverCoderV01),
      _ => Err(GameOverCoderVersionError::UnrecognisedVersion(s.clone())),
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum GameOverCoderVersionError
{
  UnrecognisedVersion(String),
}

impl Display for GameOverCoderVersionError
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    use GameOverCoderVersionError::*;

    match self {
      UnrecognisedVersion(s) => write!(f, "version '{}' is unrecognised", s),
    }
  }
}

impl Error for GameOverCoderVersionError {}

impl From<GameOverCoderVersion> for &'static str
{
  fn from(version: GameOverCoderVersion) -> Self
  {
    use GameOverCoderVersion::*;

    match version {
      Version00Coding => "00",
      GameOverCoderV01 => "goc-v01",
    }
  }
}

impl From<GameOverCoderVersion> for String
{
  fn from(version: GameOverCoderVersion) -> Self
  {
    String::from_str(version.into()).unwrap()
  }
}
