use clap::{arg, Command};
use crate::blockchain::Blockchain;
use crate::errors::Result;
use std::process::exit;
use bincode::deserialize;
use crate::block::Block;
use crate::transaction::Transaction;

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
            .subcommand(Command::new("createwallet").about("create a wallet"))
            .subcommand(Command::new("listaddresses").about("list all addresses"))
            .subcommand(Command::new("reindex").about("reindex UTXO"))
            .subcommand(Command::new("getbalance")
                .about("get balance in the blochain")
                .arg(arg!(<ADDRESS>"'The Address it get balance for'"))
            )
            .subcommand(Command::new("startnode")
                .about("start the node server")
                .arg(arg!(<PORT>"'the port server bind to locally'"))
            )
            .subcommand(Command::new("create").about("Create new blochain")
                .arg(arg!(<ADDRESS>"'The address to send gensis block reqward to' "))
            )
            .subcommand(
                Command::new("send")
                    .about("send  in the blockchain")
                    .arg(arg!(<FROM>" 'Source wallet address'"))
                    .arg(arg!(<TO>" 'Destination wallet address'"))
                    .arg(arg!(<AMOUNT>" 'Destination wallet address'"))
                    .arg(arg!(-m --mine " 'the from address mine immediately'")),
            )
            .get_matches();

        

        if let Some(_) = matches.subcommand_matches("printchain") {
            println!("printchain...");
            cmd_print_chain()?;
        }

        if let Some(ref matches) = matches.subcommand_matches("create") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                cmd_create_blockchain(address)?;
            }
        }


        if let Some(ref matches) = matches.subcommand_matches("getbalance") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                let balance = cmd_get_balance(address)?;
                println!("Balance: {}\n", balance);
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("send") {
            let from = if let Some(address) = matches.get_one::<String>("FROM") {
                address
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            let to = if let Some(address) = matches.get_one::<String>("TO") {
                address
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            let amount: i32 = if let Some(amount) = matches.get_one::<String>("AMOUNT") {
                amount.parse()?
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            if matches.contains_id("mine") {
                cmd_send(from, to, amount, true)?;
            } else {
                cmd_send(from, to, amount, false)?;
            }
        }
        Ok(())
    }
}

fn cmd_send(from: &str, to: &str, amount: i32, mine_now: bool) -> Result<()> {
    let mut bc = Blockchain::new()?;

    let tx = Transaction::new_UTXO(from,to,amount,&bc)?;

    bc.add_block_with_tx(vec![tx]);
    
    println!("success!");
    Ok(())
}

fn cmd_create_blockchain(address: &str) -> Result<()> {
    let address = String::from(address);
    let bc = Blockchain::create_blockchain(address)?;
    println!("create blockchain");
    Ok(())
}
fn cmd_get_balance(address: &str) -> Result<i32> {

    let bc = Blockchain::new()?;
    let address = String::from(address);
    let utxos = bc.find_UTXO(&address);

    println!("{:?}",utxos);

    let mut balance = 0;
    for out in utxos {
        balance += out.value;
    }
    println!("Balance of '{}'; {}", address,balance);
    Ok(balance)
}
fn cmd_print_chain() -> Result<()> {
    let bc = Blockchain::new()?;
    for b in bc.iter() {
        println!("{:#?}", b);
    }
    Ok(())
}