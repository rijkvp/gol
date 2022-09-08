use gol::{
    game::{run, GameConfig, GameMode},
    life::{LifeConfig, PatternConfig},
};

pub fn main() {
    env_logger::init();

    if let Err(e) = run(GameConfig {
        life: LifeConfig {
            size: (1024, 1024),
            pattern: PatternConfig::Random,
        },
        mode: GameMode::Pixels,
    }) {
        eprintln!("failed to run game: {}", e);
    }
}
