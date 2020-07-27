#[macro_use]
extern crate clap;

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

mod exponentiation;
mod files;
mod helpers;
mod sort;
mod sudoku;

use crossterm::Result;
use dirs::home_dir;

fn main() -> Result<()> {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Vui-Chee")
        (about: "Run rust examples")
        (@arg input: +required "Sets the input file to use")
        (@subcommand sudoku =>
            (about: "Run the sudoku example.")
        )
        (@subcommand sort =>
            (about: "Run the sorting algorithms.")
        )
        (@subcommand largest_files =>
            (about: "Get the n largest files from dir.")
        )
    )
    .get_matches();

    if let Some(_matches) = matches.subcommand_matches("sudoku") {
        sudoku::run();
    }

    if let Some(_matches) = matches.subcommand_matches("sort") {
        println!("Run some sorting algorithms.");
    }

    if let Some(_matches) = matches.subcommand_matches("largest_files") {
        if let Some(home) = home_dir() {
            let home_path = home.to_str().unwrap_or("");
            format!("{}", home_path);
            let entries = files::largest_files(home_path, 5)?;
            for entry in entries {
                println!("{:?}", entry);
            }
        }
    }

    Ok(())
}
