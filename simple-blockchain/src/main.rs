mod models;
use models::blockchain::Blockchain;
use std::{io, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Please input a difficulty: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let difficulty = input.trim().parse::<usize>()?;

    let mut blockchain = Blockchain::new(difficulty);
    Blockchain::add_block(&mut blockchain);

    Ok(())
}