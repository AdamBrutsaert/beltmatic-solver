use beltmatic::{FastSolver, FullSolver, Value};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        required = true,
        help = "The value for which the combinations must be found"
    )]
    target: Value,

    #[arg(
        short,
        long,
        value_delimiter = ',',
        default_value = "1,2,3,4,5,6,7,8,9",
        help = "The base numbers to use"
    )]
    base: Vec<Value>,

    #[arg(
        short,
        long,
        help = "Use a faster algorithm but with only one solution"
    )]
    fast: bool,
}

fn main() {
    let args = Args::parse();

    if args.fast {
        println!("{}", FastSolver::new(args.base).solve(args.target));
    } else {
        for solution in FullSolver::new(args.base).solve(args.target) {
            println!("{}", solution);
        }
    }
}
