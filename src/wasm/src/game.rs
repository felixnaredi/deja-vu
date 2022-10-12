mod game;
mod game_error;
mod unseen;

#[cfg(test)]
mod test;

pub use game::{
  Game,
  INITIAL_LIVES_AMOUNT,
};
pub use game_error::GameError;
pub use unseen::Unseen;