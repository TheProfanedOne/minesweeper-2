use crate::random::random_fields;
use std::{
    collections::HashSet,
    fmt::{Display, Write}
};

pub type Position = (usize, usize);

pub type BoardSize = (usize, usize, usize);

#[derive(PartialEq)]
pub enum OpenResult {
    Mine,
    NoMine(u8),
}

pub struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_fields: HashSet<Position>,
    lose: bool,
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let no_mine_tiles = vec!["  ", "1 ", "2 ", "3 ", "4 ", "5 ", "6 ", "7 ", "8 "];
        for y in 0..self.height {
            for x in 0..self.width {
                if !self.is_open((x, y)) {
                    f.write_str(if !self.is_flagged((x, y)) { "⬛" } else { "🚩" })?;
                } else if self.is_mine((x, y)) {
                    f.write_str("💣")?;
                } else {
                    f.write_str(no_mine_tiles[self.neighboring_mines((x, y)) as usize])?;
                }
                if x != self.width - 1 { f.write_char('\u{200B}')?; }
            }
            if y != self.height - 1 { f.write_char('\n')?; }
        }
        Ok(())
    }
}

impl Minesweeper {
    pub fn new((width, height, mine_count): BoardSize) -> Minesweeper {
        Minesweeper {
            width,
            height,
            open_fields: HashSet::new(),
            mines: {
                let mut mines = HashSet::new();

                for field in random_fields(width, height, mine_count) {
                    mines.insert(field);
                }

                mines
            },
            flagged_fields: HashSet::new(),
            lose: false,
        }
    }

    pub fn board_reset(&mut self) {
        let mine_count = self.mine_count();
        self.mines.clear();
        self.flagged_fields.clear();
        self.open_fields.clear();
        for field in random_fields(self.width, self.height, mine_count) {
            self.mines.insert(field);
        }
    }

    pub fn width_and_height(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn false_lost(&mut self) {
        self.lose = false;
    }

    pub fn lost(&mut self) {
        self.lose = true;
    }

    pub fn lose_state(&self) -> bool {
        self.lose
    }

    pub fn show_loss(&mut self, width: usize, height: usize) {
        let it = (0..width).flat_map(move |i| (0..height).map(move |j| (i, j)));
        for field in it {
            if let Some(_) = self.open(field) {}
            else {
                if let Some(flagged) = self.toggle_flag(field) {
                    if flagged { self.toggle_flag(field); }
                    self.open(field);
                }
            }
        }
    }

    pub fn mine_count(&self) -> usize {
        self.mines.len()
    }

    fn is_mine(&self, pos: Position) -> bool {
        self.mines.contains(&pos)
    }

    pub fn win_check(&self) -> bool {
        let total_fields = self.width * self.height;

        if self.flagged_fields.eq(&self.mines) {
            if self.open_fields.len() == total_fields - self.flagged_fields.len() {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn is_open(&self, pos: Position) -> bool {
        self.open_fields.contains(&pos)
    }

    pub fn is_flagged(&self, pos: Position) -> bool {
        self.flagged_fields.contains(&pos)
    }

    fn iter_neighbors(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height;

        (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |&pos| pos != (x, y))
    }

    pub fn neighboring_mines(&self, position: Position) -> u8 {
        self.iter_neighbors(position)
            .filter(move |&pos| self.is_mine(pos))
            .count() as u8
    }

    pub fn open(&mut self, position: Position) -> Option<OpenResult> {
        if self.open_fields.contains(&position) {
            let mine_count = self.neighboring_mines(position);
            let flag_count = self
                .iter_neighbors(position)
                .filter(|neighbor| self.flagged_fields.contains(neighbor))
                .count() as u8;
      
            if mine_count == flag_count {
                for neighbor in self.iter_neighbors(position) {
                    if !self.flagged_fields.contains(&neighbor)
                       && !self.open_fields.contains(&neighbor)
                    {
                        self.open(neighbor);
                    }
                }
            }
      
            return None;
        }

        if self.flagged_fields.contains(&position) {
            return None;
        }
    
        self.open_fields.insert(position);

        if self.is_mine(position) {
            self.lost();
            Some(OpenResult::Mine)
        } else {
            let mine_count = self.neighboring_mines(position);

            if mine_count == 0 {
                for neighbor in self.iter_neighbors(position) {
                    self.open(neighbor);
                }
            }

            Some(OpenResult::NoMine(mine_count))
        }
    }

    pub fn toggle_flag(&mut self, position: Position) -> Option<bool> {
        if !self.open_fields.contains(&position) {
            if !self.flagged_fields.remove(&position) {
                Some(self.flagged_fields.insert(position))
            } else { Some(false) }
        } else { None }
    }
}