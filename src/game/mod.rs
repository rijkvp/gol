use self::game::Game;
use crate::{error::Error, life::LifeConfig};

mod game;
mod renderer;
mod simul;

#[derive(Debug, Clone)]
pub struct GameConfig {
    pub life: LifeConfig,
    pub graphics: bool,
}

pub fn start(cfg: GameConfig) -> Result<(), Error> {
    Game::new(cfg)?.run();
    Ok(())
}
