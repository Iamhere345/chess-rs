/*

io.rs - parses input (e.g >ROOK E2)

*/

use std::io::{self, Write};

// TODO windows input compatability
pub fn handle_input() -> Result<String, ()> {
    let commands = [
        String::from("help"),
        String::from("display"),
        String::from("move"),
        String::from("load"),
        String::from("save"),
        String::from("quit"),
        String::from("end"),
    ];

    //print!(">");

    let mut input = String::new();

    print!("> ");
    let flush_result = io::stdout().flush();

    if flush_result.is_err() {
        return Err(());
    }

    let read_result = io::stdin().read_line(&mut input);

    if read_result.is_err() {
        // todo handle io errors properly
        println!("io error: {}", read_result.unwrap_err());
        return Err(());
    }

    let str_slice = &input.trim()[0..4];

    print!("input slice: '{}'\n", str_slice);
    println!("input: '{}'", input.trim());

    Ok(input)
}
