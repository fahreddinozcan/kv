use std::process::exit;

use clap::{Parser, Subcommand};
use kvs::KvStore;

#[derive(Parser, Debug)]
#[command(
    name = "kvs",
    version = "1.0",
    author = "Your Name",
    about = "A simple key-value CLI store"
)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}
fn main() {
    let _kv = KvStore::new();
    let args = Args::parse();

    match args.command {
        Commands::Get { key: _ } => {
            // if let Some(value) = kv.get(key) {
            // 	println!("{}", value);
            // } else {
            // 	println!("Key not found");
            // }
            eprintln!("unimplemented");
            exit(1);
        }
        Commands::Set { key: _, value: _ } => {
            // kv.set(key, value);
            eprintln!("unimplemented");
            exit(1);
        }
        Commands::Rm { key: _ } => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
