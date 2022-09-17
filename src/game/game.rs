use super::{renderer::Renderer, simul, GameConfig};
use crate::{error::Error, life::Life};
use crossbeam_channel::bounded;
use std::time::Duration;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[derive(Debug)]
pub enum GameSpeed {
    Delay(Duration),
    Max,
}

impl Default for GameSpeed {
    fn default() -> Self {
        Self::Delay(Duration::from_millis(20))
    }
}

#[derive(Debug)]
pub enum InputEvent {
    TogglePaused,
    SetSpeed(GameSpeed),
}

pub struct Game {
    life: Life,
    window: Window,
    event_loop: EventLoop<()>,
    renderer: Option<Renderer>,
}

impl Game {
    pub fn new(cfg: GameConfig) -> Result<Self, Error> {
        let life = Life::from_config(&cfg.life)?;

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let renderer = {
            if cfg.graphics {
                Some(pollster::block_on(async { Renderer::new(&window).await }))
            } else {
                None
            }
        };

        Ok(Self {
            life,
            window,
            event_loop,
            renderer,
        })
    }

    pub fn run(mut self) {
        let (state_tx, state_rx) = bounded(1);
        let (event_tx, event_rx) = bounded(1);
        let _simul_handle = simul::start_simul(self.life.clone(), state_tx, event_rx);

        self.event_loop
            .run(move |event, _, control_flow| match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(keycode),
                                ..
                            },
                        ..
                    } => match keycode {
                        VirtualKeyCode::Space => event_tx.send(InputEvent::TogglePaused).unwrap(),
                        VirtualKeyCode::Key1 => event_tx
                            .send(InputEvent::SetSpeed(GameSpeed::default()))
                            .unwrap(),
                        VirtualKeyCode::Key2 => event_tx
                            .send(InputEvent::SetSpeed(GameSpeed::Delay(
                                Duration::from_millis(10),
                            )))
                            .unwrap(),
                        VirtualKeyCode::Key3 => event_tx
                            .send(InputEvent::SetSpeed(GameSpeed::Delay(
                                Duration::from_millis(3),
                            )))
                            .unwrap(),
                        VirtualKeyCode::Key4 => {
                            event_tx.send(InputEvent::SetSpeed(GameSpeed::Max)).unwrap()
                        }
                        VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
                        _ => {}
                    },
                    WindowEvent::Resized(physical_size) => {
                        if let Some(renderer) = &mut self.renderer {
                            renderer.resize(*physical_size);
                            self.window.request_redraw();
                        }
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        if let Some(renderer) = &mut self.renderer {
                            renderer.resize(**new_inner_size);
                            self.window.request_redraw();
                        }
                    }
                    _ => {}
                },
                Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                    if let Some(renderer) = &mut self.renderer {
                        match renderer.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.size),
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                *control_flow = ControlFlow::Exit
                            }
                            Err(e) => eprintln!("render error: {:?}", e),
                        }
                    }
                }
                Event::MainEventsCleared => {
                    if let Some(renderer) = &mut self.renderer {
                        if let Ok(grid) = state_rx.try_recv() {
                            grid.draw(renderer.pixels());
                            self.window.request_redraw();
                        }
                    }
                }
                _ => {}
            });
    }
}
