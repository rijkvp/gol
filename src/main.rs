use crossbeam_channel::bounded;
use gol::{life::Life, pattern::{self, Pattern}};
use pixels::{Pixels, SurfaceTexture};
use std::{
    thread,
    time::{Duration, SystemTime},
};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

pub fn main() {
    run(512);
}

enum GameSpeed {
    Delay(Duration),
    Max,
}

impl Default for GameSpeed {
    fn default() -> Self {
        Self::Delay(Duration::from_millis(100))
    }
}

enum InputEvent {
    TogglePaused,
    SetSpeed(GameSpeed),
}

fn run(grid_size: u32) {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let size = LogicalSize::new(grid_size as f64, grid_size as f64);
    let window = WindowBuilder::new()
        .with_title("test")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(&event_loop)
        .unwrap();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(grid_size, grid_size, surface_texture).unwrap()
    };

    let (event_tx, event_rx) = bounded(1);
    let (state_tx, state_rx) = bounded(1);

    // Game thread
    thread::spawn(move || {
        let mut update_time = SystemTime::now();
        let pattern = Pattern::from_plaintext_file("pattern.cells").unwrap();
        let mut game = Life::from_pattern(grid_size as usize, &pattern);
        let mut paused = false;
        let mut speed = GameSpeed::default();
        loop {
            let delta = update_time.elapsed().unwrap();
            let tps = (1.0 / delta.as_secs_f64()).floor();
            update_time = SystemTime::now();
            if game.tick % 500 == 0 {
                println!("T {} - TPS {}", game.tick, tps);
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
    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            match event {
                Event::MainEventsCleared => {
                    if let Ok(grid) = state_rx.try_recv() {
                        grid.draw(pixels.get_frame());
                    }
                    pixels.render().unwrap();
                }
                _ => (),
            }
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed(VirtualKeyCode::Key1) {
                event_tx
                    .send(InputEvent::SetSpeed(GameSpeed::Delay(
                        Duration::from_millis(500),
                    )))
                    .unwrap();
            }
            if input.key_pressed(VirtualKeyCode::Key2) {
                event_tx
                    .send(InputEvent::SetSpeed(GameSpeed::Delay(
                        Duration::from_millis(100),
                    )))
                    .unwrap();
            }
            if input.key_pressed(VirtualKeyCode::Key3) {
                event_tx
                    .send(InputEvent::SetSpeed(GameSpeed::Delay(
                        Duration::from_millis(20),
                    )))
                    .unwrap();
            }
            if input.key_pressed(VirtualKeyCode::Key4) {
                event_tx
                    .send(InputEvent::SetSpeed(GameSpeed::Delay(
                        Duration::from_millis(5),
                    )))
                    .unwrap();
            }
            if input.key_pressed(VirtualKeyCode::Key5) {
                event_tx
                    .send(InputEvent::SetSpeed(GameSpeed::Delay(
                        Duration::from_millis(2),
                    )))
                    .unwrap();
            }
            if input.key_pressed(VirtualKeyCode::Key6) {
                event_tx
                    .send(InputEvent::SetSpeed(GameSpeed::Delay(
                        Duration::from_millis(1),
                    )))
                    .unwrap();
            }
            if input.key_pressed(VirtualKeyCode::Key0) {
                event_tx.send(InputEvent::SetSpeed(GameSpeed::Max)).unwrap();
            }
            if input.key_pressed(VirtualKeyCode::Space) {
                event_tx.send(InputEvent::TogglePaused).unwrap();
            }

            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }
            window.request_redraw();
        }
    });
}
