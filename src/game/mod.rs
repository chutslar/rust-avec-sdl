use events::{Event, EventType, EventPool};

pub struct Game<'a> {
    // Required to use events
    event_pool: &'a mut EventPool,
    // Used for example logic
    signaled: bool,
    check_signal_count: u32
}

impl<'a> Game<'a> {
    pub fn new(event_pool : &'a mut EventPool) -> Self {
        Game { event_pool, signaled: false, check_signal_count: 0}
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


        // Remove events marked for removal each frame
        self.event_pool.remove_events();
    }
}