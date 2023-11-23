use crate::board::Board;

pub struct MinesweeperModel {
    board: Board,
    status: GameStatus,
}

impl MinesweeperModel {
    pub fn new(height: i32, width: i32, mines: i32) -> MinesweeperModel {
        MinesweeperModel { board: Board::new_custom(height, width, mines), status: GameStatus::NotReady }
    }

    pub fn init_game(&mut self) {
        self.board.make_board();
        self.status = GameStatus::Ready;
    }

    pub fn start_game(&mut self) {
        
    }

    pub fn get_board(&self) -> Board {
        self.board.clone()
    }
}

pub enum GameStatus {
    NotReady,
    Ready,
    Playing,
    Won,
    Lost,
}