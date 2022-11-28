use clap::{Args, Parser, Subcommand};
use sudoku::{grid_to_string, string_to_grid, Generator, Solver};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Solve a sudoku
    Solve(Solve),
    /// Generate a sudoku
    Generate(Generate),
    /// Check that a sudoku is valid
    Check(Check),
}

#[derive(Args)]
struct Solve {
    sudoku: String,
}

#[derive(Args)]
struct Check {
    sudoku: String,
}

#[derive(Args)]
struct Generate {
    number: Option<usize>,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Solve(args) => {
            let mut solver = Solver::new();

            match string_to_grid(&args.sudoku) {
                Ok(grid) => match solver.solve(&grid) {
                    Some(solved) => {
                        let output = grid_to_string(&solved);
                        println!("{}", output);
                    }
                    None => println!("Not solveable"),
                },
                Err(error) => println!("{}", error),
            }
        }
        Commands::Check(_args) => {
            todo!();
        }
        Commands::Generate(args) => {
            let mut generator = Generator::new();
            for _ in 0..(args.number.unwrap_or(1)) {
                let sudoku = generator.generate_sudoku();
                println!("{}", grid_to_string(&sudoku));
            }
        }
    }
}
