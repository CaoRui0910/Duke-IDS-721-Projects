use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Noah Gift",
    about = "Command-line interface for ONNX"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Noah Gift")]
    SurvivalTime {
        age: u64,
    },
}

//invoke lib.rs using onnx_demo namespace
//use onnx_demo::run;
fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::SurvivalTime {age}) => {
            let result0 = 8760 * age;
            let result1 = 8760 * age * 60;
            println!("You have lived for {} hours", result0);
            println!("Or you have lived for {} minutes", result1);
        }
        None => println!("No subcommand was used"),
    }
}