use crate::{model::MinesweeperModel, textview::MinesweeperTextView};

mod model;
mod tile;
mod board;
mod textview;

fn main() {
    println!("Welcome to Minesweeper!");
    let mut model: MinesweeperModel = MinesweeperModel::new(10, 10, 15);
    model.init_game();
    println!("Poggers!");
    let view: MinesweeperTextView = MinesweeperTextView::new(model);
    view.view_board();
}

