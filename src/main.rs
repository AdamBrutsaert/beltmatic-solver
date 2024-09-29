use beltmatic::{FastSolver, Value};
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
}

fn main() {
    let args = Args::parse();
    println!("{}", FastSolver::new(args.base).solve(args.target));
}
