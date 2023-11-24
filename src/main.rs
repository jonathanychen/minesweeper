use crate::{model::MinesweeperModel, textview::MinesweeperTextView};

mod model;
mod tile;
mod board;
mod textview;

fn main() {
    println!("Welcome to Minesweeper!");
    let mut model: MinesweeperModel = MinesweeperModel::new_seeded(420);
    model.custom_board(10, 10, 25);
    model.init_game();
    let view: MinesweeperTextView = MinesweeperTextView::new(model);
    view.view_board();
}

