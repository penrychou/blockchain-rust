use crate::errors::Result;
use crate::cli::Cli;

mod block;
mod blockchain;
mod errors;
mod transaction;
mod cli;

fn main() ->Result<()> {
    let mut cli = Cli::new()?;
    println!("main run.....");
    cli.run()?;
    Ok(())
}
