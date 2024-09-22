//A command-line to play Marco 
use clap::Parser;
#[derive(Parser)]
#[clap(version = "1.0", author = "Noah Gift" )]



Struct CLi {
    [clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]

enum Commands {
    #[clap(version = "1.0", author = "Noah Gift")]
    Play {
        #[clap(short, long)]
        name: String,
    },
}

fn main() {
    let args = Cli =Cliparse();
    match args.command {
        Some(Commands::Play { name }) => {
            let result: String  = hello_marco::marco_polo(&name);
            println!("{}", result);
        }
        None => 
            println!("No command provided"),
                }
    }

