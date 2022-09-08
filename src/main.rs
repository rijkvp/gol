use std::path::PathBuf;

use clap::Parser;
use gol::{
    game::{run, GameConfig, GameMode},
    life::{LifeConfig, PatternConfig},
};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 1024)]
    width: usize,
    #[clap(short, long, default_value_t = 1024)]
    height: usize,
    #[clap(short, long, value_name = "FILE")]
    pattern: Option<PathBuf>,
    #[clap(long)]
    headless: bool,
}

impl Args {
    fn into_config(self) -> GameConfig {
        let pconfig = {
            if let Some(pfile) = self.pattern {
                PatternConfig::File(pfile)
            } else {
                PatternConfig::Random
            }
        };
        GameConfig {
            life: LifeConfig {
                size: (self.width, self.height),
                pattern: pconfig,
            },
            mode: if self.headless {
                GameMode::Headless
            } else {
                GameMode::Pixels
            },
        }
    }
}

pub fn main() {
    env_logger::init();

    let args = Args::parse();

    if let Err(e) = run(args.into_config()) {
        eprintln!("failed to run game: {}", e);
    }
}
