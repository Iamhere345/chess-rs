use std::collections::HashMap;
use std::process::ExitCode;

use game::GameHandler;
use io::handle_input;

pub mod board;
pub mod game;
pub mod io;
pub mod rules;

fn main() {
    let game = GameHandler::new(HashMap::from([
        ("Pawn", 1),
        ("Knight", 3),
        ("Bishop", 3),
        ("Rook", 5),
        ("Queen", 9),
        ("King", 0),
    ]));

    print!("type 'help' for a list of commands\nCHESS-RS version 0.1.0\n");

    while !game.game_over {
        let input_result = handle_input();

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
            game.board.display();
        } else if input_raw[0..4].eq_ignore_ascii_case("move") {
            // todo parse move input
            println!("yes");
        } else if input.eq_ignore_ascii_case("exit") {
            break;
        } else {
            println!("invalid input");
        }

    }
}
