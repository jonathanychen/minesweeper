use crate::{board::Board, model::MinesweeperModel, tile::{Value, Tile}};

pub struct MinesweeperTextView {
    model: MinesweeperModel
}

impl MinesweeperTextView {
    pub fn new(model: MinesweeperModel) -> MinesweeperTextView {
        MinesweeperTextView {
            model
        }
    }

    pub fn view_board(&self) {
        let board: Vec<Tile> = self.model.get_board();
        let width = self.model.get_width();

        let mut i = 0;
        for tile in board {
            let char = get_char(tile);
            print!("{}", char);
            i += 1;
            if i % width == 0 {
                println!();
            }
        }

        fn get_char(t: Tile) -> char {
            if !t.is_visible() {
                return '_'
            }
            let v = t.get_value();
            match v {
                Value::Mine => 'x',
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
}