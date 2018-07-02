extern crate sdl2;

use events::{Event, EventType, EventPool};
use controller::{Controllers, ControllerState};
use controller::sdl2::controller::{Axis, Button};

pub struct Game<'a> {
    // Required to use events
    event_pool: &'a mut EventPool,
    controllers: &'a Controllers,
    controller_state: ControllerState,
    // Used for example logic
    signaled: bool,
    check_signal_count: u32
}

impl<'a> Game<'a> {
    pub fn new(
        event_pool : &'a mut EventPool, 
        controllers: &'a Controllers) -> Self {
        
        Game { 
            event_pool, 
            controllers,
            controller_state: ControllerState::new(&controllers),
            signaled: false, 
            check_signal_count: 0
        }
    }

    pub fn update(&mut self, delta_time_ms: u64) {

        // Example logic using events
        if !self.signaled {
            self.event_pool.signal_event(
                EventType::OnNameEntered,
                Event::OnNameEntered {
                    trigger_id: 23, 
                    name: "Jaques"
                }
            );
            self.signaled = true;
        } else {
            let event = self.event_pool.poll_events(
                EventType::OnNameEntered,
                23,
                if self.check_signal_count > 5 { true } else { false }
            );
            match event {
                Some(e) => println!("Got event: {:?}", e),
                None => (),
            }
            self.check_signal_count += 1;
        }

        match self.controllers.button_down(0, Button::A, &self.controller_state) {
            Some(b) => if b { println!("Got A") },
            None => (),
        }

        // Remove events marked for removal each frame
        self.event_pool.remove_events();

        // Update controller state
        self.controller_state.update(self.controllers);
    }
}