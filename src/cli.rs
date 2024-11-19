use clap::{arg, Command};
use crate::blockchain::Blockchain;
use crate::errors::Result;

pub struct Cli{}

impl Cli {
    pub fn new() -> Result<Cli> {
        Ok(Cli {})
    }

    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("blockchain-rust-demo")
        .version("0.1")
        .author("penry")
        .about("blockchain in rust: a simple blockchain for learning")
        .subcommand(Command::new("printchain").about("print all the chain blocks"))
        .subcommand(Command::new("addblock")
            .about("get balance in the blochain")
            .arg(arg!(<DATA>"'the data added to the block'"))
        )
        .subcommand(Command::new("getBalance")
            .about("get balance in the blochain")
            .arg(arg!(<ADDRESS>"'The Address it get balance for'"))
        )
        .get_matches();

        if let Some(_) = matches.subcommand_matches("printchain") {
            cmd_print_chain()?;
        }

        if let Some(ref matches) = matches.subcommand_matches("addblock") {
            if let Some(data) = matches.get_one::<String>("DATA") {
                cmd_add_block(data)?;
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("getBalance") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                //let balance = cmd_get_balance(address)?;
                println!("Balance[{}]: \n", address);
            }
        }

        Ok(())
    }
}

fn cmd_print_chain() -> Result<()> {
    let bc = Blockchain::new()?;
    for b in bc.iter() {
        println!("{:#?}", b);
    }
    Ok(())
}

fn cmd_add_block(data: &str) -> Result<()> {
    let mut bc = Blockchain::new()?;
    let tx_data = String::from(data);
    // bc.add_block(tx_data);
    Ok(())
}

// fn cmd_get_balance(address: &str) -> Result<i32> {
//     let pub_key_hash = Address::decode(address).unwrap().body;
//     let bc = Blockchain::new()?;
//     let utxo_set = UTXOSet { blockchain: bc };
//     let utxos = utxo_set.find_UTXO(&pub_key_hash)?;

//     let mut balance = 0;
//     for out in utxos.outputs {
//         balance += out.value;
//     }
//     Ok(balance)
// }