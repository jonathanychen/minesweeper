use crate::{model::MinesweeperModel, tile::{Value, Tile}};

pub struct MinesweeperTextView {
    model: MinesweeperModel
}

impl MinesweeperTextView {
    pub fn new(model: MinesweeperModel) -> MinesweeperTextView {
        MinesweeperTextView {
            model
        }
    }

    fn view_board_variant(&self, char_fn: &dyn Fn(&Self, Tile) -> char) {
        let board: Vec<Tile> = self.model.get_board();
        let width = self.model.get_width();

        let mut i: i32 = 0;
        for tile in board {
            let char: char = char_fn(self, tile);
            print!("{}", char);
            i += 1;
            if i % width == 0 {
                println!();
            }
        }
    }

    pub fn view_visible_board(&self) {
        self.view_board_variant(&Self::get_visible_char);
    }

    pub fn view_true_board(&self) {
        self.view_board_variant(&Self::get_true_value);
    }

    fn get_visible_char(&self, t: Tile) -> char {
        if !t.is_visible() {
            return '_'
        }
        self.get_true_value(t)
    }

    fn get_true_value(&self, t: Tile) -> char {
        match t.get_value() {
            Value::Mine => '*',
            Value::Zero => '0',
            Value::One => '1',
            Value::Two => '2',
            Value::Three => '3',
            Value::Four => '4',
            Value::Five => '5',
            Value::Six => '6',
            Value::Seven => '7',
            Value::Eight => '8',
        }
    }
}