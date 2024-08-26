use crate::common_concept::{common_concept};
use crate::guess_game::guess_game_init;
use crate::ownership::ownership_concept;

mod guess_game;
mod common_concept;
mod ownership;

fn main() {
    println!("Hello World");
    //guess_game_init();
    //common_concept();
    ownership_concept();
}