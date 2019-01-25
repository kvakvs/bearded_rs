use crate::game::Game;
use crate::gamedef::GameDef;

mod component;
mod game;
mod gamedef;
mod world;

fn main() {
  let gamedef = GameDef::new();
  let mut game = Game::new([128, 128]);
  game.new_game();

  println!("Hello, world!");
}
