use std::collections::HashSet;

use rand::{Rng, rngs::StdRng};

use crate::tile::{Tile, Value};

pub struct MinesweeperModel {
    status: GameStatus,
    height: i32,
    width: i32,
    mines: i32,
    board: Vec<Tile>,
    num_tiles: i32,
    rng: StdRng,
}

impl MinesweeperModel {
    pub fn new() -> MinesweeperModel {
        MinesweeperModel { 
            height: 0, 
            width: 0, 
            mines: 0, 
            board: Vec::new(), 
            num_tiles: 0, 
            status: GameStatus::NotReady, 
            rng: rand::SeedableRng::seed_from_u64(rand::thread_rng().gen()) 
        }
    }

    pub fn new_seeded(seed: u64) -> MinesweeperModel {
        MinesweeperModel { 
            height: 0, 
            width: 0, 
            mines: 0, 
            board: Vec::new(), 
            num_tiles: 0, 
            status: GameStatus::NotReady, 
            rng: rand::SeedableRng::seed_from_u64(seed) 
        }
    }

    pub fn init_game(&mut self) {
        self.make_board();
        self.status = GameStatus::Ready;
    }

    pub fn start_game(&mut self) {

    }

    pub fn get_board(&self) -> Vec<Tile> {
        self.board.clone()
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    // eventually change to mines - flags
    pub fn get_mines(&self) -> i32 {
        self.mines
    }

    pub fn custom_board(&mut self, height: i32, width: i32, mines: i32) {
        self.height = height;
        self.width = width;
        self.mines = mines;
        self.num_tiles = height * width;
    }

    pub fn make_board(&mut self) {
        let chosen_spots: HashSet<i32> = self.choose_mines_any();
        self.create_board_vector(chosen_spots);
        self.update_board()
    }

    fn choose_mines_any(&mut self) -> HashSet<i32> {
        self.choose_mines_exclude_set(HashSet::new())
    }

    fn choose_mines_exclude_set(&mut self, excluded: HashSet<i32>) -> HashSet<i32> {
        let mut chosen_spots = HashSet::<i32>::new();
        let mut random_mine: i32 = self.rng.gen_range(0..self.num_tiles);
        for _ in 0..self.mines {
            while chosen_spots.contains(&random_mine) && !excluded.contains(&random_mine) {
                random_mine = self.rng.gen_range(0..self.num_tiles);
            }
            chosen_spots.insert(random_mine);
        }
        return chosen_spots;
    }

    fn create_board_vector(&mut self, mines: HashSet<i32>) {
        for r in 0..self.height {
            for c in 0..self.width {
                let index: i32 = c + r * self.width;
                self.board.push(Tile::new(c, r, mines.contains(&index)))
            }
        }
    }

    pub fn update_board(&mut self) {
        for r in 0..self.height {
            for c in 0..self.width {
                if !self.tile_is_mine(r, c) {
                    let count: i32 = self.calculate_mine_count(r, c);
                    self.update_tile(r, c, count);
                }
            }
        }
    }

    fn tile_is_mine(&mut self, r: i32, c: i32) -> bool {
        let center: usize = (r * self.width + c).try_into().unwrap();
        let current: &Tile = self.board.get(center).unwrap();
        current.get_value() == Value::Mine
    }

    fn calculate_mine_count(&mut self, r: i32, c: i32) -> i32 {
        let mut count: i32 = 0;
        for d in DIRECTIONS.clone() {
            let c2: i32 = c + d[0];
            let r2: i32 = r + d[1];
            if r2 < 0 || r2 >= self.height || c2 < 0 || c2 >= self.width {
                continue;
            }
            let i: usize = (r2 * self.width + c2) as usize;
            match self.board.get(i) {
                Some(tile) => if tile.get_value() == Value::Mine {
                    count += 1
                },
                None => continue,
            };
        }
        count
    }

    fn update_tile(&mut self, r: i32, c: i32, count: i32) {
        let center: usize = (r * self.width + c).try_into().unwrap();
        let current: &mut Tile = &mut self.board.get_mut(center).unwrap();
        current.set_value(Tile::get_associated_value(count).unwrap());
    }
}

pub enum GameStatus {
    NotReady,
    Ready,
    Playing,
    Won,
    Lost,
}

static DIRECTIONS: &[&[i32]] = &[&[-1, -1], &[-1, 0], &[-1, 1], &[0, -1], &[0, 1], &[1, -1], &[1, 0], &[1, 1]];