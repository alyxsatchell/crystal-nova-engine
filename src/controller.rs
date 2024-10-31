use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

use crate::object::Object;
enum Command {
    Up,
    Down,
    Left,
    Right,
    RightRotate,
    LeftRotate
}

struct InputController{
    // maybe a key binding map
    commands: Vec<Command>,
    speed: f32,
}

impl InputController {
    fn new() -> Self {
        Self { commands: Vec::new(), speed: 0.1}
    }

    fn process(&mut self, event: &WindowEvent) -> bool {

        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state,
                        physical_key: PhysicalKey::Code(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    KeyCode::KeyW | KeyCode::ArrowUp => {
                        self.commands.push(Command::Up);
                        true
                    }
                    KeyCode::KeyA | KeyCode::ArrowLeft => {
                        self.commands.push(Command::Left);
                        true
                    }
                    KeyCode::KeyS | KeyCode::ArrowDown => {
                        self.commands.push(Command::Down);
                        true
                    }
                    KeyCode::KeyD | KeyCode::ArrowRight => {
                        self.commands.push(Command::Right);
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn update(&self, object: &mut dyn Object) {
        for command in &self.commands{
            match command{
                Command::Up => {
                    object.Up();
                },
                Command::Down => {
                    object.Down();
                },
                Command::Left => {
                    object.Left();
                },
                Command::Right => {
                    object.Right();
                },
                Command::RightRotate => {
                    todo!()
                },
                Command::LeftRotate => {
                    todo!()
                },
            }
        }
    }

}