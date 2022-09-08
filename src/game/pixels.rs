use super::{GameSpeed, InputEvent};
use crate::{grid::Grid, life::LifeConfig};
use crossbeam_channel::{Receiver, Sender};
use pixels::{Pixels, SurfaceTexture};
use std::time::Duration;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

pub(super) fn run_pixels(cfg: &LifeConfig, state_rx: Receiver<Grid>, event_tx: Sender<InputEvent>) {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let size = LogicalSize::new(cfg.size.0 as f64, cfg.size.1 as f64);
    let window = WindowBuilder::new()
        .with_title("test")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(&event_loop)
        .unwrap();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(cfg.size.0 as u32, cfg.size.1 as u32, surface_texture).unwrap()
    };

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
