use std::collections::HashMap;

use winit::{
    event::*,
    keyboard::{KeyCode, PhysicalKey},
};

use crate::object::Object;

#[derive(Debug,PartialEq, Eq, Hash, Clone, Copy)]
pub enum Command {
    Up,
    Down,
    Left,
    Right,
    RightRotate,
    LeftRotate
}

pub struct InputController{
    // maybe a key binding map
    commands: HashMap<Command, bool>,
    key_binds: HashMap<KeyCode, Command>,
    speed: f32,
}

impl InputController {
    pub fn new() -> Self {
        let commands_list = [Command::Up, Command::Down, Command::Left, Command::Right];
        let input_list = [KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD];
        let mut commands = HashMap::new();
        for command in commands_list{
            commands.insert(command, false);
        }
        let mut key_binds = HashMap::new();
        let mut i = 0;
        for input in input_list{
            key_binds.insert(input, commands_list[i as usize]);
            i += 1;
        }
        Self { commands, key_binds, speed: 0.001}
    }

    pub fn process(&mut self, event: &WindowEvent) -> bool {
        let mut return_val = false;
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
                return_val = is_pressed;
                println!("{:?}", keycode);
                let command_code = self.key_binds.get(keycode);
                match command_code{
                    Some(command) => {self.commands.insert(*command, is_pressed);},
                    None => (),
                }
                }
            _ => ()
        }
        return return_val;
    }

    pub fn update(&mut self, object: &mut dyn Object) {
        let length = self.commands.len();
        if !(length == 0)
        {
            println!("{}", self.commands.len());
        }
        for key in &self.commands{
            if *key.1{
                let command = key.0;
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

}