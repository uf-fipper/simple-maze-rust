#![cfg(feature = "console")]

use clap::{CommandFactory, FromArgMatches};
use maze::{
    console_game::{self, Cli, ConsoleGame, SubcommandNew},
    errors::MazeResult,
    game::Game,
};
use rand::{thread_rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

fn run() -> MazeResult<()> {
    let cli;
    if std::env::args().len() == 1 {
        cli = Cli::New(SubcommandNew {
            row: 10,
            column: 20,
            seed: None,
        });
    } else {
        let mut matches = match console_game::Cli::command()
            .multicall(false)
            .try_get_matches()
        {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e.to_string());
                return Ok(());
            }
        };
        cli = match console_game::Cli::from_arg_matches_mut(&mut matches) {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e.to_string());
                return Ok(());
            }
        }
    }
    let mut game = match cli {
        Cli::New(SubcommandNew { row, column, seed }) => {
            let random = match seed {
                Some(state) => ChaCha8Rng::seed_from_u64(state),
                None => ChaCha8Rng::from_rng(thread_rng()).unwrap(),
            };
            let game = ConsoleGame::new_with_random(row, column, random);
            game
        }
        _ => {
            println!("new game is only allow new command");
            return Ok(());
        }
    }?;
    game.run()
}

fn main() {
    match run() {
        Err(e) => println!("{}", e.to_string()),
        _ => {}
    }
}
