use std::collections::HashSet;
use rand::prelude::*;

use crate::tile::Tile;

#[derive(Clone)]
pub struct Board {
    height: i32,
    width: i32,
    mines: i32,
    grid: Vec<Tile>,
    num_tiles: i32,
}

impl Board {
    pub fn new_custom(height: i32, width: i32, mines: i32) -> Board {
        Board {
            height,
            width,
            mines,
            grid: Vec::new(),
            num_tiles: height * width,
        }
    }

    pub fn make_board(&mut self) {
        // helper
        let mut chosen_spots: HashSet<i32> = HashSet::<i32>::new();
        let mut random_mine: i32 = rand::thread_rng().gen_range(0..self.num_tiles);
        for _ in 0..self.mines {
            while chosen_spots.contains(&random_mine) {
                random_mine = rand::thread_rng().gen_range(0..self.num_tiles);
            }
            chosen_spots.insert(random_mine);
        }

        // let mut rng = thread_rng();
        // let v: Vec<i32> = (0..self.num_tiles).collect();
        // let chosen_spots: Vec<i32> = v.choose_multiple(&mut rng, self.mines.try_into().unwrap()).collect();

        println!("{:?}", chosen_spots);

        // helper
        for r in 0..self.height {
            for c in 0..self.width {
                let index: i32 = c * r;
                self.grid.push(Tile::new(c, r, chosen_spots.contains(&index)))
            }
        }

        // println!("{:?}", self.grid);

        // one more helper for updating tiles

    }

    pub fn get_info(&self) -> (i32, i32, Vec<Tile>) {
        (   
            self.height,
            self.width,
            self.grid.clone(),
        )
    }
}

enum Directions {

}