/*

input.rs - Handles input (duh)

*/

use std::io::{self, Write};

// TODO windows input compatability
pub fn handle_input(prompt: String) -> Result<String, ()> {
    //print!(">");

    let mut input = String::new();

    print!("{}", prompt);
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

    Ok(input)
}
