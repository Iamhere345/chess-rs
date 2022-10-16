/*

game.rs - handles current game state

*/

use std::collections::HashMap;

use crate::board::*;
use crate::rules::can_move_peice;

pub struct GameHandler {
    turn: Team,
    pub board: Board,
    pub points_map: HashMap<&'static str, i32>,
    pub black_points: i32,
    pub white_points: i32,
    pub game_over: bool,
}

impl GameHandler {
    pub fn new(points_map: HashMap<&'static str, i32>) -> GameHandler {
        GameHandler {
            turn: Team::White,
            board: Board::new(Team::Black),
            points_map: points_map,
            black_points: 0,
            white_points: 0,
            game_over: false,
        }
    }

    pub fn end_turn(mut self) {
        self.turn = self.turn.flip();
    }

    pub fn take_peice(&mut self, peice_coord: &BoardCoord) -> Result<(), String> {
        if peice_coord.x > 7 || peice_coord.y > 7 {
            return Err("Invalid coordinates".to_string());
        }

        if self.board.board[peice_coord.x][peice_coord.y].is_none() {
            return Err("Attempt to take peice that doesn't exist".to_string());
        }

        let taken_peice = self.board.board[peice_coord.x][peice_coord.y].unwrap();

        let points = match taken_peice.peice_type {
            PeiceType::Pawn => self.points_map.get("Pawn").unwrap_or(&0),
            PeiceType::Rook => self.points_map.get("Rook").unwrap_or(&0),
            PeiceType::Knight => self.points_map.get("Knight").unwrap_or(&0),
            PeiceType::Bishop => self.points_map.get("Bishop").unwrap_or(&0),
            PeiceType::King => self.points_map.get("King").unwrap_or(&0),
            PeiceType::Queen => self.points_map.get("Queen").unwrap_or(&0),
            PeiceType::Empty => &0,
        };

        self.board.board[peice_coord.x][peice_coord.y] = None;

        match self.turn {
            Team::Black => self.black_points += points,
            Team::White => self.white_points += points,
            _ => return Err("Turn should not be neutral".to_string()),
        }

        Ok(())
    }

    pub fn move_peice(
        mut self,
        peice_coord: BoardCoord,
        move_coord: BoardCoord,
    ) -> Result<(), String> {
        if peice_coord.x > 7 || peice_coord.y > 7 {
            return Err("Invalid peice coordinate".to_string());
        }

        if move_coord.x > 7 || move_coord.y > 7 {
            return Err("Invalid move coordinate".to_string());
        }

        let peice_opt = self.board.board[peice_coord.x][peice_coord.y];

        // stops players from moving empty squares
        if peice_opt.is_none() {
            return Err("No peice at coordinate".to_string());
        } else {
            let mut peice = peice_opt.unwrap();

            if peice.team != self.turn {
                return Err("Cannot move opponent's peice".to_string());
            }

            let can_move_peice = can_move_peice(peice);

            if can_move_peice.is_err() {
                return Err(can_move_peice.unwrap_err());
            }

            // checks if the peice at the given coord is the same team
            if self.board.board[move_coord.x][move_coord.y].is_some()
                && self.board.board[move_coord.x][move_coord.y].unwrap().team == self.turn
            {
                return Err("Cannot take your own peice".to_string());
            }

            // checks if the peice at the target coord is the opposite team
            if self.board.board[move_coord.x][move_coord.y].is_some()
                && self.board.board[move_coord.x][move_coord.y].unwrap().team != self.turn
            {
                let take_result = self.take_peice(&move_coord);

                if take_result.is_err() {
                    return Err(take_result.unwrap_err());
                }

                self.board.board[move_coord.x][move_coord.y] = Some(peice);

                self.end_turn();
            }
        }

        Ok(())
    }
}
