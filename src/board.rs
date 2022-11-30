/*

board.rs - sets up the board and provides access to the peices

*/

//TODO peices go in wrong direction

use std::{fmt, ops::Add};

// types of chess peices
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PeiceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Team {
    Black,
    White,
    Neutral,
}

impl Team {
    pub fn flip(self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
            Self::Neutral => Self::Neutral,
        }
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Black => write!(f, "Black"),
            Self::White => write!(f, "White"),
            Self::Neutral => write!(f, "Neutral"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
}

impl Direction {
    fn flip(self) -> Self {
        match self {
            Self::Down => Self::Up,
            Self::Up => Self::Down
        }
    }
}

// this used to index board array, which is why it needs usize numbers
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BoardCoord {
    pub x: usize,
    pub y: usize,
}

impl BoardCoord {
    pub fn new(_x: usize, _y: usize) -> BoardCoord {
        BoardCoord { x: _x, y: _y }
    }
}

impl Add for BoardCoord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl From<(i32, i32)> for BoardCoord {

    //! The following code is will result in an integer wrap-around if the raw_coord i32s are negative
    fn from(raw_coord: (i32, i32)) -> Self {

        let x = raw_coord.0;
        let y = raw_coord.1;

        return Self {
            x: x as usize,
            y: y as usize,
        }
        
    }
}

#[derive(Clone, Copy)]
pub struct Board {
    pub board: [[Option<Peice>; 8]; 8],
    pub ai_team: Team,
}

impl Board {
    pub fn new(_ai_team: Team) -> Board {
        Board {
            board: fill_board(),
            ai_team: _ai_team,
        }
    }

    pub fn display(self) {
        // collumns
        for i in (0..9).rev() {
            if i != 0 {
                print!("{} ", i);
            }

            if i == 0 {
                print!("  ");

                for letter in BOARD_X_LETTERS.iter() {
                    print!("{}", letter);
                }
                continue;
            }

            // rows
            for (x, row) in self.board.iter().enumerate() {
                let board_i = i - 1;

                if row[board_i].is_none() {
                    //let board_square: &str;

                    //print!("{}", x + i);

                    if (x + board_i) % 2 == 0 {
                        print!("\u{25A0}"); // white square
                    } else {
                        print!("\u{25A1}"); // black square
                    }
                } else {
                    let peice = row[board_i].unwrap();

                    let peice_str: &str;

                    // TODO teams are wrong unicode symbols
                    if peice.team == Team::White {
                        peice_str = match peice.peice_type {
                            PeiceType::Pawn => "\u{265F}",
                            PeiceType::Rook => "\u{265C}",
                            PeiceType::Bishop => "\u{265D}",
                            PeiceType::Knight => "\u{265E}",
                            PeiceType::King => "\u{265A}",
                            PeiceType::Queen => "\u{265B}",
                        };
                    } else {
                        peice_str = match peice.peice_type {
                            PeiceType::Pawn => "\u{2659}",
                            PeiceType::Rook => "\u{2656}",
                            PeiceType::Bishop => "\u{2657}",
                            PeiceType::Knight => "\u{2658}",
                            PeiceType::King => "\u{2654}",
                            PeiceType::Queen => "\u{265B}",
                        };
                    }

                    print!("\x1B[1m{}\x1B[0m", peice_str);
                    
                }
                //print!(" ");
            }
            print!("\n")
        }
        println!();
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Peice {
    pub peice_type: PeiceType,
    pub team: Team,
    pub dir: Direction,
    pub x_pos: usize,
    pub y_pos: usize,
}

impl Peice {
    fn new(x: usize, y: usize, _peice_type: PeiceType, _team: Team, _dir: Direction) -> Peice {
        Peice {
            peice_type: _peice_type,
            team: _team,
            dir: _dir,
            x_pos: x,
            y_pos: y,
        }
    }
}

/*
in chess, the board positions are determined on one axis with a letter and the other with a number,
to convert the chess positions to a human readable format, we can index this array with the x position.
*/

pub const BOARD_X_LETTERS: [char; 8] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

fn fill_board() -> [[Option<Peice>; 8]; 8] {
    /*

        board layout:

        A B C D E F G H
    1   R K B Q K B K R
    2   P P P P P P P P
    3
    4
    5
    6
    7   P P P P P P P P
    8   R K B K Q B K R

    */

    // first layer is row second layer is collumns
    let board: [[Option<Peice>; 8]; 8];
    board = [
        [
            Some(Peice::new(
                0,
                0,
                PeiceType::Rook,
                Team::White,
                Direction::Down,
            )),
            Some(Peice::new(
                0,
                1,
                PeiceType::Pawn,
                Team::White,
                Direction::Down,
            )),
            None,
            None,
            None,
            None,
            Some(Peice::new(
                0,
                6,
                PeiceType::Pawn,
                Team::Black,
                Direction::Up,
            )),
            Some(Peice::new(
                0,
                7,
                PeiceType::Rook,
                Team::Black,
                Direction::Up,
            )),
        ],
        [
            Some(Peice::new(
                1,
                0,
                PeiceType::Knight,
                Team::White,
                Direction::Down,
            )),
            Some(Peice::new(
                1,
                1,
                PeiceType::Pawn,
                Team::White,
                Direction::Down,
            )),
            None,
            None,
            None,
            None,
            Some(Peice::new(
                1,
                6,
                PeiceType::Pawn,
                Team::Black,
                Direction::Up,
            )),
            Some(Peice::new(
                1,
                7,
                PeiceType::Knight,
                Team::Black,
                Direction::Up,
            )),
        ],
        [
            Some(Peice::new(
                2,
                0,
                PeiceType::Bishop,
                Team::White,
                Direction::Down,
            )),
            Some(Peice::new(
                2,
                1,
                PeiceType::Pawn,
                Team::White,
                Direction::Down,
            )),
            None,
            None,
            None,
            None,
            Some(Peice::new(
                2,
                6,
                PeiceType::Pawn,
                Team::Black,
                Direction::Up,
            )),
            Some(Peice::new(
                2,
                7,
                PeiceType::Bishop,
                Team::Black,
                Direction::Up,
            )),
        ],
        [
            Some(Peice::new(
                3,
                0,
                PeiceType::Queen,
                Team::White,
                Direction::Down,
            )),
            Some(Peice::new(
                3,
                1,
                PeiceType::Pawn,
                Team::White,
                Direction::Down,
            )),
            None,
            None,
            None,
            None,
            Some(Peice::new(
                3,
                6,
                PeiceType::Pawn,
                Team::Black,
                Direction::Up,
            )),
            Some(Peice::new(
                3,
                7,
                PeiceType::King,
                Team::Black,
                Direction::Up,
            )),
        ],
        [
            Some(Peice::new(
                4,
                0,
                PeiceType::King,
                Team::White,
                Direction::Down,
            )),
            Some(Peice::new(
                4,
                1,
                PeiceType::Pawn,
                Team::White,
                Direction::Down,
            )),
            None,
            None,
            None,
            None,
            Some(Peice::new(
                4,
                6,
                PeiceType::Pawn,
                Team::Black,
                Direction::Up,
            )),
            Some(Peice::new(
                4,
                7,
                PeiceType::Queen,
                Team::Black,
                Direction::Up,
            )),
        ],
        [
            Some(Peice::new(
                5,
                0,
                PeiceType::Bishop,
                Team::White,
                Direction::Down,
            )),
            Some(Peice::new(
                5,
                1,
                PeiceType::Pawn,
                Team::White,
                Direction::Down,
            )),
            None,
            None,
            None,
            None,
            Some(Peice::new(
                5,
                6,
                PeiceType::Pawn,
                Team::Black,
                Direction::Up,
            )),
            Some(Peice::new(
                5,
                7,
                PeiceType::Bishop,
                Team::Black,
                Direction::Up,
            )),
        ],
        [
            Some(Peice::new(
                6,
                0,
                PeiceType::Knight,
                Team::White,
                Direction::Down,
            )),
            Some(Peice::new(
                6,
                1,
                PeiceType::Pawn,
                Team::White,
                Direction::Down,
            )),
            None,
            None,
            None,
            None,
            Some(Peice::new(
                6,
                6,
                PeiceType::Pawn,
                Team::Black,
                Direction::Up,
            )),
            Some(Peice::new(
                6,
                7,
                PeiceType::Knight,
                Team::Black,
                Direction::Up,
            )),
        ],
        [
            Some(Peice::new(
                7,
                0,
                PeiceType::Rook,
                Team::White,
                Direction::Down,
            )),
            Some(Peice::new(
                7,
                1,
                PeiceType::Pawn,
                Team::White,
                Direction::Down,
            )),
            None,
            None,
            None,
            None,
            Some(Peice::new(
                7,
                6,
                PeiceType::Pawn,
                Team::Black,
                Direction::Up,
            )),
            Some(Peice::new(
                7,
                7,
                PeiceType::Rook,
                Team::Black,
                Direction::Up,
            )),
        ],
    ];

    board
}
