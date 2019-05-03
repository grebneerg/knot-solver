use structopt::StructOpt;
use knot_solver::Knot;
use std::str::FromStr;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(subcommand)]
    command: Command
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "bracket")]
    Bracket {
        braid: String,
    }
}

fn main() {
    let opt = Opt::from_args();

    match opt.command {
        Command::Bracket { braid } => println!("{}", Knot::from_str(braid.as_str()).unwrap().bracket_polynomial()),
    }
}
