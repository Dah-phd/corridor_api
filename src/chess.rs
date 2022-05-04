extern crate rocket;
use crate::abstarctions::{GameSession, PlayerMove, PlayerMoveResult};
use rocket::fs::{relative, FileServer};
use rocket::serde::{Deserialize, Serialize};

struct Chess {
    // (row, column)
    black_king: (usize, usize),
    black_queen: (usize, usize),
    black_pawns: Vec<(usize, usize)>,
    black_knights: Vec<(usize, usize)>,
    black_horses: Vec<(usize, usize)>,
    black_rooks: Vec<(usize, usize)>,
    white_king: (usize, usize),
    white_queen: (usize, usize),
    white_pawns: Vec<(usize, usize)>,
    white_knights: Vec<(usize, usize)>,
    white_horses: Vec<(usize, usize)>,
    white_rooks: Vec<(usize, usize)>,
}

impl Chess {
    pub fn new() -> Self {
        Self {
            black_king: (0, 4),
            black_queen: (0, 3),
            black_pawns: (0..8).map(|x| (1, x)).collect(),
            black_knights: vec![(0, 2), (0, 5)],
            black_horses: vec![(0, 1), (0, 6)],
            black_rooks: vec![(0, 0), (0, 7)],
            white_king: (7, 4),
            white_queen: (7, 3),
            white_pawns: (0..8).map(|x| (6, x)).collect(),
            white_knights: vec![(7, 2), (7, 5)],
            white_horses: vec![(7, 1), (7, 6)],
            white_rooks: vec![(7, 0), (7, 7)],
        }
    }
    pub fn move_player(&mut self, from: (usize, usize), new_position: (usize, usize)) -> bool {
        let from_color = self.get_color(&from);
        let to_color = self.get_color(&new_position);
        if from_color == "" {
            return false;
        }
        if from_color == to_color {
            return false;
        }
        if !self.is_possible(&from, &new_position) {
            return false;
        }
        true
    }

    fn is_possible(&self, from: &(usize, usize), to: &(usize, usize)) -> bool {
        true
    }

    fn get_color(&self, position: &(usize, usize)) -> &str {
        if self.black_king == *position
            || self.black_queen == *position
            || self.black_pawns.contains(position)
            || self.black_knights.contains(position)
            || self.black_horses.contains(position)
            || self.black_rooks.contains(position)
        {
            return "b";
        }
        if self.white_king == *position
            || self.white_queen == *position
            || self.white_pawns.contains(position)
            || self.white_knights.contains(position)
            || self.white_horses.contains(position)
            || self.white_rooks.contains(position)
        {
            return "w";
        }
        ""
    }
}

struct ChessSession {}
