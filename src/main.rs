use beltmatic::FastSolver;
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
    target: i64,

    #[arg(
        short,
        long,
        value_delimiter = ',',
        default_value = "1,2,3,4,5,6,7,8,9",
        help = "The base numbers to use"
    )]
    base: Vec<i64>,
}

fn main() {
    let args = Args::parse();
    let mut solver = FastSolver::new(args.base);
    println!("{}", solver.solve(args.target));
}
