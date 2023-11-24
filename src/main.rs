use crate::{model::MinesweeperModel, textview::MinesweeperTextView};

mod model;
mod tile;
mod textview;

fn main() {
    println!("Welcome to Minesweeper!");
    let mut model: MinesweeperModel = MinesweeperModel::new_seeded(420);
    model.custom_board(10, 10, 25);
    model.init_game();
    let view: MinesweeperTextView = MinesweeperTextView::new(model);
    println!("True board:");
    view.view_true_board();
    println!("Visible board:");
    view.view_visible_board();
}

