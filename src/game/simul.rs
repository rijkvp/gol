use super::game::{GameSpeed, InputEvent};
use crate::{grid::Grid, life::Life};
use crossbeam_channel::{Receiver, Sender};
use log::info;
use std::{
    thread::{self, JoinHandle},
    time::SystemTime,
};

pub fn start_simul(
    mut life: Life,
    state_tx: Sender<Grid>,
    event_rx: Receiver<InputEvent>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut update_time = SystemTime::now();
        let mut paused = false;
        let mut speed = GameSpeed::default();
        loop {
            let delta = update_time.elapsed().unwrap();
            let tps = (1.0 / delta.as_secs_f64()).floor();
            update_time = SystemTime::now();
            if life.tick % 50 == 0 {
                info!("T {} - TPS {}", life.tick, tps);
            }
            if let Ok(event) = event_rx.try_recv() {
                info!("receive input {:?}", event);
                match event {
                    InputEvent::TogglePaused => paused = !paused,
                    InputEvent::SetSpeed(s) => speed = s,
                }
            }
            if !paused {
                life.update();
            }
            if let GameSpeed::Delay(delay) = speed {
                thread::sleep(delay);
            }
            // This is kinda ugly but it works
            if let Err(_e) = state_tx.try_send(life.get_state().clone()) {}
        }
    })
}
