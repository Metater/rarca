mod guess_the_number;
mod tic_tac_toe;

use tic_tac_toe::TicTacToe;

fn main() {
    println!("Hello, world!");
    TicTacToe::play();
    guess_the_number::play();
}