use std::collections::HashMap;

use game::GameHandler;
use input::handle_input;

use crate::board::BoardCoord;

pub mod board;
pub mod game;
pub mod input;
pub mod rules;

fn main() {
    let mut game = GameHandler::new(HashMap::from([
        ("Pawn", 1),
        ("Knight", 3),
        ("Bishop", 3),
        ("Rook", 5),
        ("Queen", 9),
        ("King", 0),
    ]));

    print!("type 'help' for a list of commands\nCHESS-RS version 0.1.0\n");

    'main: while !game.is_over() {
        let input_result = handle_input(format!("{}>", game.playing_team()));

        if input_result.is_err() {
            continue;
        }

        let input_raw = input_result.unwrap();
        let input = input_raw.trim();

        //println!("str: {}", input.as_str());

        if input.eq_ignore_ascii_case("help") {
            print!(
                "\n Command list:\n
                help: display this message\n
                display: display the board on the screen\n
                move <Peice Coord> <New Coord>: move a peice to the given coordinate\n
                load <save name>: load a previous game state\n
                save <save name>: save the current game state\n
                exit: kill the program\n
                end: surrender the current game, making your opponent the winner\n"
            );
        } else if input.eq_ignore_ascii_case("display") {
            game.clone().board.display();
        } else if input.len() >= 4 && input[0..4].eq_ignore_ascii_case("move") {
            // todo parse move input

            if input.len() < "MOVE A0 A0".len() {
                println!("error: did not input exactly 2 arguments (move syntax: MOVE <PEICE COORD> <NEW COORD> e.g: MOVE A1 H2");
                continue;
            }

            let peice_arg = &input[5..7];
            let dest_arg = &input[8..10];

            fn convert_input_to_char(arg: &&str) -> Result<(usize, usize), ()> {
                let mut x_output: usize = 0;
                let mut y_output: usize = 0;

                for (i, v) in arg.chars().enumerate() {
                    if i == 0 && v.is_numeric() {
                        println!("Error: expected chess co-ordinate (e.g A1 or C3)");
                        return Err(());
                    } else if i == 0 && v.is_alphabetic() {
                        // there might be a better way to do this. Just sayin'
                        if v.eq_ignore_ascii_case(&'A') {
                            x_output = 0;
                        } else if v.eq_ignore_ascii_case(&'B') {
                            x_output = 1;
                        } else if v.eq_ignore_ascii_case(&'C') {
                            x_output = 2;
                        } else if v.eq_ignore_ascii_case(&'D') {
                            x_output = 3;
                        } else if v.eq_ignore_ascii_case(&'E') {
                            x_output = 4;
                        } else if v.eq_ignore_ascii_case(&'F') {
                            x_output = 5;
                        } else if v.eq_ignore_ascii_case(&'G') {
                            x_output = 6;
                        } else if v.eq_ignore_ascii_case(&'H') {
                            x_output = 7;
                        } else {
                            println!("Error: a chess board only extends to co-ordinate 'H'");
                            return Err(());
                        }
                    }

                    if i == 1 && v.is_numeric() {
                        let int_result = v.to_string().parse::<usize>();
                        let char_as_int: usize;

                        if int_result.is_err() {
                            println!("Error: Y co-ordinate must be a number");
                            return Err(());
                        } else {
                            char_as_int = int_result.unwrap();
                        }

                        if char_as_int > 8 {
                            println!("Error: a chess board only extends to co-ordinate 8");
                            return Err(());
                        }

                        if char_as_int == 0 {
                            println!("Error: a chess board's y axis begins at 1, not 0");
                            return Err(());
                        }

                        y_output = char_as_int - 1;
                    }
                }

                Ok((x_output, y_output))
            }

            let mut peice_x: usize = 0;
            let mut peice_y: usize = 0;

            let mut dest_x: usize = 0;
            let mut dest_y: usize = 0;

            let peice_result = convert_input_to_char(&peice_arg);
            let dest_result = convert_input_to_char(&dest_arg);

            if peice_result.is_err() || dest_result.is_err() {
                continue 'main;
            } else if peice_result.is_ok() && dest_result.is_ok() {
                let peice_result_unwrapped = peice_result.unwrap();
                let dest_result_unwrapped = dest_result.unwrap();

                peice_x = peice_result_unwrapped.0;
                peice_y = peice_result_unwrapped.1;

                dest_x = dest_result_unwrapped.0;
                dest_y = dest_result_unwrapped.1;
            }

            let move_result = game.move_peice(
                BoardCoord::new(peice_x, peice_y),
                BoardCoord::new(dest_x, dest_y),
            );

            if move_result.is_err() {
                println!("Unable to move peice: {}.", move_result.unwrap_err());
            }
        } else if input.eq_ignore_ascii_case("exit") {
            println!("goodbye");
            break;
        } else if input.len() >= 5 && input[0..5].eq_ignore_ascii_case("debug") {
            let x_slice = &input[6..7];
            let y_slice = &input[8..9];

            let mut x: usize = 0;
            let mut y: usize = 0;

            for char in x_slice.chars() {
                if char.is_numeric() {
                    let parse_result = char.to_string().parse::<usize>();

                    if parse_result.is_err() {
                        println!("number expected");
                        continue 'main;
                    } else {
                        x = parse_result.unwrap();
                        break;
                    }
                }
                continue 'main;
            }

            for char in y_slice.chars() {
                if char.is_numeric() {
                    let parse_result = char.to_string().parse::<usize>();

                    if parse_result.is_err() {
                        println!("number expected");
                        continue 'main;
                    } else {
                        y = parse_result.unwrap();
                        break;
                    }
                }
                continue 'main;
            }

            if x <= 7 && y <= 7 {
                println!(
                    "peice at position {}-{}: {:?}",
                    x, y, game.board.board[x][y]
                );
            } else {
                println!("index out bounds");
            }
        } else if input.eq_ignore_ascii_case("skip") {
            
            println!("ending turn...");

            game.end_turn();

        } else if input.eq("") {
            continue;
        } else {
            println!("invalid input");
        }
    }
}
