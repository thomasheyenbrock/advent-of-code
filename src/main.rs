mod cli;
mod puzzles;

fn main() {
    let puzzles = cli::parse_args();

    for puzzle in puzzles {
        puzzle.solve();
    }

    // let input = read_input(Year::_2025, Day::_01)?;
    // y2025::d01::run(&input);
}
