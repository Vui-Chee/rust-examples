#[macro_use]
extern crate clap;

mod helpers;
mod sudoku;

use crossterm::Result;

fn main() -> Result<()> {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Vui-Chee")
        (about: "Run rust examples")
        (@arg input: +required "Sets the input file to use")
        (@subcommand sudoku =>
            (about: "Run the sudoku example.")
        )
    )
    .get_matches();

    if let Some(_matches) = matches.subcommand_matches("sudoku") {
        sudoku::run();
    }

    Ok(())
}
