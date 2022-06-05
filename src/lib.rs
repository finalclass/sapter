use rand::prelude::*;
use std::fmt::{Display, Formatter};
use std::vec::Vec;

type Pos = (usize, usize);

#[derive(Debug)]
pub struct GameState {
    mines: Vec<Pos>,
    opened: Vec<Pos>,
    flags: Vec<Pos>,
    pub player: Pos,
    pub width: usize,
    pub height: usize,
    is_alive: bool,
}

#[derive(Debug, Clone)]
pub struct RangeError;

impl Display for RangeError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "RangeError: specified position is out of range")
    }
}

impl GameState {
    pub fn new(width: usize, height: usize, nof_mines: usize) -> GameState {
        GameState {
            mines: generate_mines(width, height, nof_mines),
            opened: Vec::new(),
            flags: Vec::new(),
            player: (0, 0),
            width,
            height,
            is_alive: true,
        }
    }

    fn open(&mut self, x: usize, y: usize) -> Result<(), RangeError> {
        self.is_in_range(x, y)?;
        if !self.is_opened(x, y) {
            self.opened.push((x, y));
        }

        Ok(())
    }

    pub fn flag(&mut self) -> Result<(), RangeError> {
        let x = self.player.0;
        let y = self.player.1;

        if !self.is_alive || self.is_opened(x, y) {
            return Ok(());
        }

        if self.is_flagged(x, y) {
            self.flags.retain(|&pos| pos != (x, y));
        } else {
            self.flags.push((x, y));
        }

        Ok(())
    }

    fn is_in_range(&self, x: usize, y: usize) -> Result<(), RangeError> {
        if x > self.width || y > self.height {
            return Err(RangeError);
        }

        Ok(())
    }

    pub fn has_mine(&self, x: usize, y: usize) -> bool {
        self.mines.contains(&(x, y))
    }

    pub fn move_player_right(&mut self) {
        if self.is_alive && self.player.0 < self.width - 1 {
            self.player.0 += 1;
        }
    }

    pub fn move_player_left(&mut self) {
        if self.is_alive && self.player.0 > 0 {
            self.player.0 -= 1;
        }
    }

    pub fn move_player_up(&mut self) {
        if self.is_alive && self.player.1 > 0 {
            self.player.1 -= 1;
        }
    }

    pub fn move_player_down(&mut self) {
        if self.is_alive && self.player.1 < self.height - 1 {
            self.player.1 += 1;
        }
    }

    pub fn resurect(&mut self) {
        self.is_alive = true;
    }

    pub fn reveal(&mut self) -> Result<(), RangeError> {
        if !self.is_alive {
            return Ok(());
        }

        let x = self.player.0;
        let y = self.player.1;

        if self.is_flagged(x, y) {
            return Ok(());
        }

        if self.has_mine(x, y) {
            self.open(x, y).unwrap();
            self.is_alive = false;
            self.show_all_mines();
            return Ok(());
        }

        self.open(x, y).unwrap();
        self.recursive_open(x, y);

        Ok(())
    }

    fn recursive_open(&mut self, x: usize, y: usize) {
        let nof_mines_around = self.count_mines_around(x, y);

        if nof_mines_around > 0 {
            self.open(x, y).unwrap();
            return;
        }

        if self.has_mine(x, y) {
            return;
        }

        if self.is_flagged(x, y) {
            return;
        }

        self.open(x, y).unwrap();

        if x > 0 && self.is_in_range(x - 1, y).is_ok() && !self.is_opened(x - 1, y) {
            self.recursive_open(x - 1, y);
        }

        if y > 0 && self.is_in_range(x, y - 1).is_ok() && !self.is_opened(x, y - 1) {
            self.recursive_open(x, y - 1);
        }

        if self.is_in_range(x + 1, y).is_ok() && !self.is_opened(x + 1, y) {
            self.recursive_open(x + 1, y);
        }

        if self.is_in_range(x, y + 1).is_ok() && !self.is_opened(x, y + 1) {
            self.recursive_open(x, y + 1);
        }

        // let min_x = if x == 0 { 0 } else { x - 1 };
        // let min_y = if y == 0 { 0 } else { y - 1 };
        // let max_x = if x >= self.width { self.width } else { x + 1 };
        // let max_y = if y >= self.height { self.height } else { y + 1 };

        // for x_it in min_x..=max_x {
        //     for y_it in min_y..=max_y {
        //         if !self.is_opened(x_it, y_it) {
        //             self.open(x_it, y_it);
        //             // self.recursive_open(x_it, y_it);
        //         }
        //     }
        // }
    }

    pub fn mines_left(&self) -> usize {
        self.mines.len() - self.flags.len()
    }

    fn show_all_mines(&mut self) {
        let mines = self.mines.clone();
        for m in mines.iter() {
            self.open(m.0, m.1).unwrap();
        }
    }

    pub fn check_has_won(&mut self) -> bool {
        if self.flags.len() != self.mines.len() {
            return false;
        }

        let mut mines = self.mines.clone();
        let mut closed = self.closed();

        mines.sort();
        closed.sort();

        let c = closed
            .iter()
            .zip(mines.iter())
            .filter(|&(a, b)| a == b)
            .count();

        let has_won = c == self.mines.len();

        if has_won {
            self.is_alive = false;
        }

        has_won
    }

    fn closed(&self) -> Vec<Pos> {
        self.all()
            .into_iter()
            .filter(|pos| !self.is_opened(pos.0, pos.1) || self.has_mine(pos.0, pos.1))
            .collect()
    }

    fn all(&self) -> Vec<Pos> {
        (0..self.width)
            .map(|x| (0..self.height).map(move |y| (x, y)))
            .into_iter()
            .flatten()
            .collect()
    }

    pub fn is_player(&self, x: usize, y: usize) -> bool {
        self.player == (x, y)
    }

    pub fn is_opened(&self, x: usize, y: usize) -> bool {
        self.opened.contains(&(x, y))
    }

    pub fn is_flagged(&self, x: usize, y: usize) -> bool {
        self.flags.contains(&(x, y))
    }

    pub fn count_mines_around(&self, x: usize, y: usize) -> u8 {
        let min_x = if x == 0 { 0 } else { x - 1 };
        let min_y = if y == 0 { 0 } else { y - 1 };
        let max_x = if x >= self.width { self.width } else { x + 1 };
        let max_y = if y >= self.height { self.height } else { y + 1 };

        let mut mines_around = 0;

        for x_it in min_x..=max_x {
            for y_it in min_y..=max_y {
                if (x_it, y_it) == (x, y) {
                    continue;
                }
                if self.has_mine(x_it, y_it) {
                    mines_around += 1
                }
            }
        }

        mines_around
    }

    // fn render_field(&self, pos: Pos) -> String {
    //     if self.is_opened(pos) {
    //         if self.has_mine(pos) {
    //             return String::from("💣 ");
    //         }

    //         let nof_neighbors = self.count_mines_around(pos);

    //         if nof_neighbors == 0 {
    //             return String::from("   ");
    //         }
    //         return format!(" {} ", nof_neighbors);
    //     }

    //     if self.is_flagged(pos) {
    //         return String::from("🚩 ");
    //     }

    //     String::from("🏽 ")
    // }
}

// impl Display for GameState {
//     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
//         let mut text = String::new();

//         for x in 0..self.width {
//             for y in 0..self.height {
//                 text += &self.render_field((x, y));
//             }
//             text = text + "\n";
//         }

//         write!(f, "{}", text)
//     }
// }

fn generate_mines(width: usize, height: usize, nof_mines: usize) -> Vec<Pos> {
    let mut mines = Vec::new();

    let mut rng = rand::thread_rng();

    loop {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        let mine = (x, y);

        if !mines.contains(&mine) {
            mines.push(mine)
        }

        if mines.len() >= nof_mines {
            break;
        }
    }

    mines
}
