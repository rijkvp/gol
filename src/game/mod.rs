use crate::{
    error::Error,
    life::{Life, LifeConfig},
};
use crossbeam_channel::bounded;
use log::info;
use std::{
    thread,
    time::{Duration, SystemTime},
};

mod pixels;

pub enum GameMode {
    Headless,
    Pixels,
}

pub struct GameConfig {
    pub life: LifeConfig,
    pub mode: GameMode,
}

enum GameSpeed {
    Delay(Duration),
    Max,
}

impl Default for GameSpeed {
    fn default() -> Self {
        Self::Delay(Duration::from_millis(20))
    }
}

enum InputEvent {
    TogglePaused,
    SetSpeed(GameSpeed),
}

pub fn run(cfg: GameConfig) -> Result<(), Error> {
    let (event_tx, event_rx) = bounded(1);
    let (state_tx, state_rx) = bounded(1);


    // Game thread
    let mut game = Life::from_config(&cfg.life)?;
    thread::spawn(move || {
        let mut update_time = SystemTime::now();
        let mut paused = false;
        let mut speed = GameSpeed::default();
        loop {
            let delta = update_time.elapsed().unwrap();
            let tps = (1.0 / delta.as_secs_f64()).floor();
            update_time = SystemTime::now();
            if game.tick % 500 == 0 {
                info!("T {} - TPS {}", game.tick, tps);
            }
            if let Ok(event) = event_rx.try_recv() {
                match event {
                    InputEvent::TogglePaused => paused = !paused,
                    InputEvent::SetSpeed(s) => speed = s,
                }
            }
            if !paused {
                game.update();
            }
            if let GameSpeed::Delay(delay) = speed {
                thread::sleep(delay);
            }
            // This is kinda ugly but it works
            if let Err(_e) = state_tx.try_send(game.get_state().clone()) {}
        }
    });

    match cfg.mode {
        GameMode::Headless => (),
        GameMode::Pixels => pixels::run_pixels(&cfg.life, state_rx, event_tx),
    }

    Ok(())
}
