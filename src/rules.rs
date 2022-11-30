/*

rules.rs - defines how peices move and what is illegal

*/

use crate::board::*;

#[derive(Debug, PartialEq)]
pub enum RayHit {
    Edge,
    Peice, // RayHit::Peice only applies to peices of the same team (because you can't take your own peice). Peices on the other team will be taken by the take_peice funtion in game.rs
    None,
}

#[derive(Debug)]
pub struct RaycastResult {
    position: BoardCoord,
    hit: RayHit,
    segments: Vec<BoardCoord>,
}

impl RaycastResult {
    pub fn has_ray_passed_coord(&self, coord: &&BoardCoord) -> bool {
        for passed_coord in self.segments.iter() {
            if *passed_coord == **coord {
                return true;
            }
        }

        return false;
    }
}

struct Ray {
    pos: BoardCoord,
    dir: (i32, i32),
    segments: Vec<BoardCoord>,
}

impl Ray {
    pub fn new(_pos: BoardCoord, _dir: (i32, i32)) -> Ray {

        println!("new ray dir: ({}, {}) new ray origin: {:#?}", _dir.0, _dir.1, _pos);

        Ray {
            pos: _pos,
            dir: _dir,
            segments: Vec::new(),
        }
    }

    // TODO
    pub fn advance(&mut self) -> Result<(), ()> {

        println!("dir: {} position_x: {}", self.dir.0, self.pos.x);

        if self.dir.0 < 0 {
            // raycast x would be out of bounds
            if self.pos.x <= 0 {
                return Err(())
            }

            self.pos.x -= 1;
        } else if self.dir.0 > 0 {
            self.pos.x += 1;
        }

        if self.dir.1 < 0 {
            // raycast y would be out of bounds
            if self.pos.y <= 0 {
                return Err(())
            }

            self.pos.y -= 1;
        } else if self.dir.0 > 0 {
            self.pos.y += 1;
        }

        /*if self.dir.0 <= 0 {
            println!("sub_X");
            println!("attempting to sub 1 from usize value {}", self.pos.x);
            //self.pos.x -= (self.dir.0 * -1) as usize;
            self.pos.x -= 1;
        } else if self.dir.0 > 0 {
            println!("sub_X");
            //self.pos.x += self.dir.0 as usize;
            self.pos.x += 1;
        }

        println!("dir: {} position_y: {}", self.dir.0, self.pos.y);

        if self.dir.1 <= 0 {
            println!("sub_Y");
            //self.pos.y -= (self.dir.1 * -1) as usize;
            self.pos.y -= 1;
        } else if self.dir.1 > 0 {
            println!("add_Y");
            //self.pos.y += self.dir.1 as usize
            self.pos.y += 1;
        }*/

        Ok(())
    }
}

// TODO
pub fn can_move_peice(board: &Board, peice: &Peice, move_coord: &BoardCoord) -> Result<(), ()> {
    let x = peice.x_pos;
    let y = peice.y_pos;

    let dir = match peice.dir {
        Direction::Up => -1,
        Direction::Down => 1,
    };

    println!("peice dir: {:?}", peice.dir);

    let origin = BoardCoord::new(x, y);

    match peice.peice_type {
        PeiceType::Pawn => {
            // take diagonally
            let take_left = board_raycast(board, peice.team, origin, (1, 1 * dir), 1);

            if take_left.hit == RayHit::Peice && take_left.position == *move_coord {
                return Ok(());
            }

            let take_right = board_raycast(board, peice.team, origin, (-1, 1 * dir), 1);

            if take_right.hit == RayHit::Peice && take_left.position == *move_coord {
                return Ok(());
            }

            // go forward
            let result = board_raycast(board, peice.team, origin, (0, 1 * dir), 1);

            if result.hit == RayHit::None && *move_coord == result.position {
                return Ok(());
            }

            // go forward two squares
            if (peice.team == Team::White && x == 1) || (peice.team == Team::Black && x == 6) {
                let result_first_move =
                    board_raycast(board, peice.team, BoardCoord::new(x, y), (0, 1 * dir), 2);

                if result_first_move.hit == RayHit::None && *move_coord == result.position {
                    return Ok(());
                }
            }
        }

        PeiceType::Bishop => {
            let move_upper_right = board_raycast(board, peice.team, origin, (1, 1 * dir), 8);

            if move_upper_right.has_ray_passed_coord(&move_coord) {
                return Ok(());
            }

            let move_lower_right = board_raycast(board, peice.team, origin, (1, -1 * dir), 8);

            if move_lower_right.has_ray_passed_coord(&move_coord) {
                return Ok(());
            }

            let move_upper_left = board_raycast(board, peice.team, origin, (-1, 1 * dir), 8);

            if move_upper_left.has_ray_passed_coord(&move_coord) {
                return Ok(());
            }

            let move_lower_left = board_raycast(board, peice.team, origin, (-1, -1 * dir), 8);

            if move_lower_left.has_ray_passed_coord(&move_coord) {
                return Ok(());
            }
        }

        PeiceType::Rook => {
            let move_up = board_raycast(board, peice.team, origin, (0, 1 * dir), 8);

            if move_up.has_ray_passed_coord(&move_coord) {
                return Ok(());
            }

            let move_down = board_raycast(board, peice.team, origin, (0, -1 * dir), 8);

            if move_down.has_ray_passed_coord(&move_coord) {
                return Ok(());
            }

            let move_right = board_raycast(board, peice.team, origin, (1, 0), 8);

            if move_right.has_ray_passed_coord(&move_coord) {
                return Ok(());
            }

            let move_left = board_raycast(board, peice.team, origin, (-1, 0), 8);

            if move_left.has_ray_passed_coord(&move_coord) {
                return Ok(());
            }
        }

        PeiceType::Knight => {
            // moving up and down in an 'L' shape
            let move_upper_left = BoardCoord::from((-1, 2 * dir));
            let move_lower_left = BoardCoord::from((1, -2 * dir));
            let move_upper_right = BoardCoord::from((1, 2 * dir));
            let move_lower_right = BoardCoord::from((-1, -2 * dir));

            //moving left and right in a sideways 'L' shape
            let move_left_upper = BoardCoord::from((-2, 1 * dir));
            let move_right_upper = BoardCoord::from((2, 1 * dir));
            let move_right_lower = BoardCoord::from((2, -1 * dir));
            let move_left_lower = BoardCoord::from((-2, -1 * dir));

            let results = check_squares(
                board,
                &[
                    origin + move_upper_left,
                    origin + move_upper_right,
                    origin + move_lower_right,
                    origin + move_lower_left,
                    origin + move_left_upper,
                    origin + move_right_upper,
                    origin + move_right_lower,
                    origin + move_left_lower,
                ],
                peice.team,
            );

            for result in results.iter() {
                if result.has_ray_passed_coord(&move_coord) {
                    return Ok(());
                }
            }
        }

        PeiceType::King => {

            // move diagonal up right
            let move_dur = board_raycast(board, peice.team, origin, (1, 1 * dir), 1);

            if move_dur.has_ray_passed_coord(&move_coord) {
                return Ok(());
            }

            let move_dul = board_raycast(board, peice.team, origin, (-1, 1 * dir), 1);

            if move_dul.has_ray_passed_coord(&move_coord) {
                return Ok(());
            }

            // move diagonal down right
            let move_ddr = board_raycast(board, peice.team, origin, (1, -1 * dir), 1);

            if move_ddr.has_ray_passed_coord(&move_coord) {
                return Ok(());
            }

            let move_ddl = board_raycast(board, peice.team, origin, (-1, -1 * dir), 1);

            if move_ddl.has_ray_passed_coord(&move_coord) {
                return Ok(());
            }

            let move_up = board_raycast(board, peice.team, origin, (0, 1 * dir), 1);

            if move_up.has_ray_passed_coord(&move_coord) {
                return Ok(());
            }

            let move_down = board_raycast(board, peice.team, origin, (0, -1 * dir), 1);

            if move_down.has_ray_passed_coord(&move_coord) {
                return Ok(());
            }

            let move_right = board_raycast(board, peice.team, origin, (1, 0), 1);

            if move_right.has_ray_passed_coord(&move_coord) {
                return Ok(());
            }

            let move_left = board_raycast(board, peice.team, origin, (-1, 0), 1);

            if move_left.has_ray_passed_coord(&move_coord) {
                return Ok(());
            }

        }

        PeiceType::Queen => {

            // move diagonal up right
            let move_dur = board_raycast(board, peice.team, origin, (1, 1 * dir), 8);

            if move_dur.has_ray_passed_coord(&move_coord) {
                return Ok(());
            }

            let move_dul = board_raycast(board, peice.team, origin, (-1, 1 * dir), 8);

            if move_dul.has_ray_passed_coord(&move_coord) {
                return Ok(());
            }

            // move diagonal down right
            let move_ddr = board_raycast(board, peice.team, origin, (1, -1 * dir), 8);

            if move_ddr.has_ray_passed_coord(&move_coord) {
                return Ok(());
            }

            let move_ddl = board_raycast(board, peice.team, origin, (-1, -1 * dir), 8);

            if move_ddl.has_ray_passed_coord(&move_coord) {
                return Ok(());
            }

            let move_up = board_raycast(board, peice.team, origin, (0, 1 * dir), 8);

            if move_up.has_ray_passed_coord(&move_coord) {
                return Ok(());
            }

            let move_down = board_raycast(board, peice.team, origin, (0, -1 * dir), 8);

            if move_down.has_ray_passed_coord(&move_coord) {
                return Ok(());
            }

            let move_right = board_raycast(board, peice.team, origin, (1, 0), 8);

            if move_right.has_ray_passed_coord(&move_coord) {
                return Ok(());
            }

            let move_left = board_raycast(board, peice.team, origin, (-1, 0), 8);

            if move_left.has_ray_passed_coord(&move_coord) {
                return Ok(());
            }

        }

    }

    Err(())
}

fn board_raycast(
    board: &Board,
    team: Team,
    origin: BoardCoord,
    direction: (i32, i32),
    range: u8,
) -> RaycastResult {
    let mut ray = Ray::new(origin, direction);

    for _ in 0..range {

        let advance_result = ray.advance();

        if advance_result.is_err() {
            // ray was out of bounds (i.e would have been a negative coord), which is is lower edge
            println!("negative coord");
            return RaycastResult {
                position: ray.pos,
                hit: RayHit::Edge,
                segments: ray.segments
            }
        }

        if ray.pos.x > 7 || ray.pos.y > 7 {
            println!("edge");
            return RaycastResult {
                position: ray.pos,
                hit: RayHit::Edge,
                segments: ray.segments,
            };
        }

        if board.board[ray.pos.x][ray.pos.y].is_some()
            && board.board[ray.pos.x][ray.pos.y].unwrap().team == team
        {
            println!("hit peice");
            return RaycastResult {
                position: ray.pos,
                hit: RayHit::Peice,
                segments: ray.segments,
            };
        }

        ray.segments.push(ray.pos);
        
    }

    // ray did not hit anything
    println!("hit nothing");
    RaycastResult {
        position: ray.pos,
        hit: RayHit::None,
        segments: ray.segments,
    }
}

// note to self: programming late at night isn't a good idea
fn check_squares<'a>(
    board: &Board,
    squares: &'a [BoardCoord],
    team: Team,
) -> Vec<RaycastResult> {
    let mut results: Vec<RaycastResult> = Vec::new();

    for coord in squares.iter() {
        let x = coord.x;
        let y = coord.y;

        if x > 7 || y > 7 {
            results.push(RaycastResult {
                position: *coord,
                hit: RayHit::Edge,
                segments: vec![*coord],
            });
        }

        if board.board[x][y].is_some() && board.board[x][y].unwrap().team == team {
            results.push(RaycastResult {
                position: *coord,
                hit: RayHit::Peice,
                segments: vec![*coord],
            })
        }
    }

    println!("results: {:?}", results);

    return results;
}
