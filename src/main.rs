use clap::{Args, Parser, Subcommand};
use sudoku::{grid_to_string, possible_to_string, string_to_grid, Generator, Solver};

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
struct Generate {}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Solve(args) => {
            let mut solver = Solver::new();

            let maybe_grid = string_to_grid(&args.sudoku);
            match maybe_grid {
                Ok(grid) => {
                    let possible = solver.grid_to_possible(&grid);
                    let maybe_solved = solver.solve(&possible);

                    match maybe_solved {
                        Some(solved) => {
                            let output = possible_to_string(&solved);
                            println!("{}", output);
                        }
                        None => println!("Not solveable"),
                    }
                }
                Err(error) => println!("{}", error),
            }
        }
        Commands::Check(_args) => {
            todo!();
        }
        Commands::Generate(_args) => {
            let mut generator = Generator::new();
            let sudoku = generator.generate_sudoku();
            println!("{}", grid_to_string(&sudoku));
        }
    }
}
