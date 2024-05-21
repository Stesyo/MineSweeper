use rand::Rng;
use std::{collections::HashSet, mem::swap, usize};

#[derive(Debug)]
pub enum Action {
    Flag,
    Dig,
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub count: u8,
    pub mine: bool,
    pub flagged: bool,
    pub digged: bool,
}

impl Tile {
    fn new() -> Tile {
        Tile {
            count: 0,
            mine: false,
            flagged: false,
            digged: false,
        }
    }
}

#[derive(Debug)]
pub struct Board {
    pub height: usize,
    pub width: usize,
    pub mine_count: usize,
    pub flags: usize,
    pub tiles_left: usize,
    pub generated: bool,
    pub alive: bool,
    pub tiles: Vec<Vec<Tile>>,
    pub to_dig: HashSet<usize>,
}

impl Board {
    // Init fresh board
    pub fn new(width: usize, height: usize, mine_count: usize) -> Board {
        let tiles = vec![vec![Tile::new(); width]; height];
        let to_dig = HashSet::new();
        Board {
            width,
            height,
            mine_count,
            flags: 0,
            tiles_left: width * height,
            generated: false,
            alive: true,
            tiles,
            to_dig,
        }
    }

    // Return list of valid tiles around target
    fn neighbours(&self, index: usize) -> Vec<usize> {
        let mut result = Vec::new();

        let x = (index % self.width) as i32;
        let y = (index / self.width) as i32;
        for dy in (y - 1)..=(y + 1) {
            for dx in (x - 1)..=(x + 1) {
                if dx == x && dy == y {
                    continue;
                }

                if self.width as i32 > dx && dx >= 0 && self.height as i32 > dy && dy >= 0 {
                    result.push(dy as usize * self.width + dx as usize);
                }
            }
        }
        result
    }

    // Generate board with respect to first mined tile
    pub fn generate(&mut self, digged_tile: usize) -> Result<(), String> {
        if self.width * self.height - 1 < self.mine_count {
            return Err("Not enouth tiles on the board".to_string());
        }

        let mut unmined_tiles = (0..(self.width * self.height)).collect::<Vec<usize>>();
        unmined_tiles.remove(digged_tile);
        let mut rng = rand::thread_rng();
        for _ in 0..self.mine_count {
            let r_tile = rng.gen_range(0..unmined_tiles.len());
            let index = unmined_tiles.remove(r_tile);

            self.tiles[index / self.width][index % self.width].mine = true;

            for n_index in self.neighbours(index) {
                self.tiles[n_index / self.width][n_index % self.width].count += 1;
            }
        }
        self.generated = true;
        self.to_dig.insert(digged_tile);
        Ok(())
    }

    // Perform outside action on board
    pub fn action(&mut self, index: usize, action: Action) {
        let tile = &self.tiles[index / self.width][index % self.width];
        match (action, tile) {
            (Action::Flag, _) => {
                let tile = &mut self.tiles[index / self.width][index % self.width];
                if !tile.digged {
                    tile.flagged = !tile.flagged;
                    if tile.flagged {
                        self.flags += 1;
                    } else {
                        self.flags -= 1;
                    }
                }
            }

            (Action::Dig, Tile { flagged: true, .. }) => {}

            (Action::Dig, Tile { digged: true, .. }) => {
                if tile.count == 0 {
                    return;
                }

                let n_flags = self
                    .neighbours(index)
                    .into_iter()
                    .map(|i| self.tiles[i / self.width][i % self.width].flagged as u8)
                    .sum::<u8>();
                if n_flags == tile.count {
                    for n_index in self.neighbours(index) {
                        self.to_dig.insert(n_index);
                    }
                }
            }

            (Action::Dig, Tile { digged: false, .. }) => {
                self.to_dig.insert(index);
            }
        }
    }

    // Advance state of the board
    pub fn tick(&mut self) {
        let mut to_dig = HashSet::new();
        swap(&mut self.to_dig, &mut to_dig);

        for index in to_dig.into_iter() {
            let tile = &mut self.tiles[index / self.width][index % self.width];
            if tile.digged || tile.flagged {
                continue;
            }

            tile.digged = true;
            self.tiles_left -= 1;
            if tile.mine {
                self.alive = false;
            }
            if tile.count == 0 {
                for n_index in self.neighbours(index) {
                    self.action(n_index, Action::Dig);
                }
            }
        }
    }
}
