#![allow(dead_code)]

pub extern crate sdl2;

use self::sdl2::GameControllerSubsystem;
use controller::sdl2::controller::{Axis, Button, GameController};

pub struct Controllers {
    pool: Vec<GameController>,
}

impl Controllers {
    pub fn new(controller_subsystem: &GameControllerSubsystem) -> Self {
        let available =
            match controller_subsystem.num_joysticks() {
                Ok(n) => n,
                Err(_e) => 0,
            };
        let mut result = Controllers { 
            pool: Vec::new() 
        };
        for id in 0..available {
            if controller_subsystem.is_game_controller(id) {
                match controller_subsystem.open(id) {
                    Ok(c) => {
                        result.pool.push(c);
                    }
                    Err(_e) => (),
                }
            }
        }
        result
    }

    pub fn any(&self) -> bool {
        self.pool.len() > 0
    }

    pub fn exists(&self, number: usize) -> bool {
        number < self.pool.len()
    }

    pub fn num(&self) -> usize {
        return self.pool.len();
    }

    pub fn name(&self, number: usize) -> Option<String> {
        if number < self.pool.len() {
            return Some(self.pool[number].name());
        }
        None
    }

    pub fn mapping(&self, number: usize) -> Option<String> {
        if number < self.pool.len() {
            return Some(self.pool[number].mapping());
        }
        None
    }

    pub fn attached(&self, number: usize) -> Option<bool> {
        if number < self.pool.len() {
            return Some(self.pool[number].attached());
        }
        None
    }

    pub fn instance_id(&self, number: usize) -> Option<i32> {
        if number < self.pool.len() {
            return Some(self.pool[number].instance_id());
        }
        None
    }

    pub fn axis(&self, number: usize, axis: Axis) -> Option<i16> {
        if number < self.pool.len() {
            return Some(self.pool[number].axis(axis));
        }
        None
    }

    pub fn button(&self, number: usize, button: Button) -> Option<bool> {
        if number < self.pool.len() {
            return Some(self.pool[number].button(button));
        }
        None
    }

    pub fn button_down(&self, number: usize, 
                        button: Button, state: &ControllerState) 
                        -> Option<bool> {
        match self.button(number, button) {
            Some(down) => {
                match state.button(number, button) {
                    Some(was_down) => Some(down && !was_down),
                    None => None,
                }
            }
            None => None,
        }
    }

    pub fn button_up(&self, number: usize, 
                        button: Button, state: &ControllerState) 
                        -> Option<bool> {
        match self.button(number, button) {
            Some(down) => {
                match state.button(number, button) {
                    Some(was_down) => Some(!down && was_down),
                    None => None,
                }
            }
            None => None,
        }
    }
}

#[derive(Clone)]
struct ButtonState(u16);

impl ButtonState {
    fn new() -> Self {
        ButtonState(0)
    }

    fn activate(&mut self, button: Button) {
        self.0 = self.0 | ((1 as u16) << button as i32);
    }
    
    fn deactivate(&mut self, button: Button) {
        self.0 = self.0 & !((1 as u16) << button as i32);
    }
    
    fn button(&self, button: Button) -> bool {
        self.0 & ((1 as u16) << button as i32) > 0
    }
}

pub struct ControllerState {
    buttons: Vec<ButtonState>,
}

impl ControllerState {
    pub fn new(controllers: &Controllers) -> Self {
        let mut result = ControllerState { buttons: Vec::new()};
        result.buttons.resize(controllers.num(), ButtonState::new());
        result
    }

    pub fn button(&self, number: usize, button: Button) -> Option<bool> {
        if number < self.buttons.len() {
            return Some(self.buttons[number].button(button));
        }
        None
    }

    pub fn update(&mut self, controllers: &Controllers) {
        for i in 0..controllers.num() {
            let mut tally : u16 = 0;
            if controllers.button(i, Button::A).unwrap() { 
                tally += 1; }
            if controllers.button(i, Button::B).unwrap() { 
                tally += 2; }
            if controllers.button(i, Button::X).unwrap() { 
                tally += 4; }
            if controllers.button(i, Button::Y).unwrap() { 
                tally += 8; }
            if controllers.button(i, Button::Back).unwrap() { 
                tally += 16; }
            if controllers.button(i, Button::Guide).unwrap() { 
                tally += 32; }
            if controllers.button(i, Button::Start).unwrap() { 
                tally += 64; }
            if controllers.button(i, Button::LeftStick).unwrap() { 
                tally += 128; }
            if controllers.button(i, Button::RightStick).unwrap() { 
                tally += 256; }
            if controllers.button(i, Button::LeftShoulder).unwrap() { 
                tally += 512; }
            if controllers.button(i, Button::RightShoulder).unwrap() { 
                tally += 1024; }
            if controllers.button(i, Button::DPadUp).unwrap() { 
                tally += 2048; }
            if controllers.button(i, Button::DPadDown).unwrap() { 
                tally += 4096; }
            if controllers.button(i, Button::DPadLeft).unwrap() { 
                tally += 8192; }
            if controllers.button(i, Button::DPadRight).unwrap() { 
                tally += 16384; }
            self.buttons[i].0 = tally;
        }
    }
}