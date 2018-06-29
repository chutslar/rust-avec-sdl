use std::collections::HashSet;

// Contains all possible events
#[derive(Debug)]
pub enum Event {
    // Example event for sending a name
    OnNameEntered{ trigger_id: u32, name: &'static str}
}

// Contains all possible event types
//  NOTE: this is expected to be in sync with both Event above and LAST_EVENT_TYPE below
pub enum EventType {
    OnNameEntered = 0,    
}

// Make sure to update this as EventTypes are added
const LAST_EVENT_TYPE: usize = EventType::OnNameEntered as usize +1;

// Stores all events in data, along with a HashSet for marking events for removal
pub struct EventPool { 
    data : [Vec<Event>; LAST_EVENT_TYPE],
    events_to_remove: HashSet<(usize, usize)>,    
}

impl EventPool {
    pub fn new() -> Self {
        EventPool { 
            data: [Vec::new(); LAST_EVENT_TYPE],
            events_to_remove: HashSet::new(),
        }
    }
    
    // Adds an event to the pool
    pub fn signal_event(&mut self, event_type: EventType, event: Event) {
        self.data[event_type as usize].push(event);
    }
    
    // Returns the first event of the proper type and with the proper id, and marks it for removal if remove_event is true
    pub fn poll_events(&mut self, event_type: EventType, target_id: u32, remove_event: bool) -> Option<&Event> {
        let mut result = None;
        
        let mut event_idx = None;

        let event_type_id = event_type as usize;

        {
            let vec = &self.data[event_type_id];

            for idx in 0..vec.len() {
                let event = &vec[idx];
                match event {
                    Event::OnNameEntered {trigger_id, name } => {
                        if target_id == *trigger_id { 
                            result = Some(event);
                            event_idx = Some(idx);
                            break;
                        }
                    }
                }
            }
        }

        if remove_event && event_idx.is_some() {
            self.events_to_remove.insert((event_type_id, event_idx.unwrap()));
        }
        result
    }

    // Removes all events currently marked for removal from the pool
    pub fn remove_events(&mut self) {
        for (event_type_id, event_idx) in &self.events_to_remove {
            self.data[*event_type_id].swap_remove(*event_idx);
        }
        self.events_to_remove.clear();
    }

    // Removes all events, whether or not they are marked for removal
    pub fn clear_all_events(&mut self) {
        for i in 0..LAST_EVENT_TYPE {
            unsafe {
                self.data[i].set_len(0);
            }
        }
    }
}
