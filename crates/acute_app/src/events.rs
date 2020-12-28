use std::marker::PhantomData;
use legion::*;
use legion::systems::Resource;
use std::fmt;
// I really like the way events work in Beavy, so Acute Events are close to a 1:1 copy from Beavy.

pub struct EventId<T> {
    pub id: usize,
    marker: PhantomData<T>,
}

impl<T> Copy for EventId<T> {}
impl<T> Clone for EventId<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> fmt::Display for EventId<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

impl<T> fmt::Debug for EventId<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "event<{}>#{}",
            std::any::type_name::<T>().split("::").last().unwrap(),
            self.id,
        )
    }
}

/// Event<T> represents any Event T
pub struct Event<T> {
    pub id: EventId<T>,
    pub event: T,
}

#[derive(Debug)]
enum State {
    A,
    B,
}

impl<T> fmt::Display for Event<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

impl<T> fmt::Debug for Event<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Event<{}>#{}",
            std::any::type_name::<T>().split("::").last().unwrap(),
            self.id
        )
    }
}

/// Events<T> is a "buffer" over all events of type T from the last 2 ticks / iterations
/// if an Event<T> is not handled in the last 2 ticks, they are dropped.
#[derive(Debug)]
pub struct Events<T> {
    buffer_a: Vec<Event<T>>,
    buffer_b: Vec<Event<T>>,
    a_start_event_count: usize,
    b_start_event_count: usize,
    event_count: usize,
    state: State,
}

impl<T> Default for Events<T> {
    fn default() -> Self {
        Self {
            buffer_a: Default::default(),
            buffer_b: Default::default(),
            a_start_event_count: 0,
            b_start_event_count: 0,
            event_count: 0,
            state: State::A,
        }
    }
}

impl<T: Resource> Events<T> {
    pub fn send(&mut self, event: T) {
        let event_id = EventId {
            id: self.event_count,
            marker: PhantomData,
        };

        let event = Event { id: event_id, event };

        match self.state {
            State::A => self.buffer_a.push(event),
            State::B => self.buffer_b.push(event),
        }

        self.event_count += 1;
    }

    // create a new [EventReader] containing all events already in the event buffers
    pub fn get_reader(&self) -> EventReader<T> {
        EventReader {
            last_event_count: 0,
            marker: PhantomData,
        }
    }

    // create a new [EventReader] skipping current events.
    pub fn get_reader_current(&self) -> EventReader<T> {
        EventReader {
            last_event_count: self.event_count,
            marker: PhantomData,
        }
    }

    // swap active buffer and clears the oldest
    pub fn update(&mut self) {
        match self.state {
            State::A => {
                self.buffer_a = Vec::new();
                self.state = State::B;
                self.b_start_event_count = self.event_count;
            }
            State::B => {
                self.buffer_b = Vec::new();
                self.state = State::A;
                self.a_start_event_count = self.event_count;
            }
        }
    }

    pub fn clear(&mut self) {
        self.buffer_a.clear();
        self.buffer_b.clear();
    }

    pub fn drain(&mut self) -> impl Iterator<Item=T> + '_ {
        let event_map = |e: Event<T>| e.event;
        match self.state {
            State::A => self
                .buffer_b
                .drain(..)
                .map(event_map)
                .chain(self.buffer_a.drain(..).map(event_map)),
            State::B => self
                .buffer_a
                .drain(..)
                .map(event_map)
                .chain(self.buffer_b.drain(..).map(event_map)),
        }
    }

    pub fn extend<I: Iterator<Item=T>>(&mut self, events: I) {
        for event in events {
            self.send(event);
        }
    }

    pub fn iter_current_update_events(&self) -> impl DoubleEndedIterator<Item = &T> {
        match self.state {
            State::A => self.buffer_a.iter().map(map_event),
            State::B => self.buffer_b.iter().map(map_event),
        }
    }
}


#[system]
pub fn event_update<T: Resource>(#[resource] events: &mut Events<T>) {
    events.update();
}


pub struct EventReader<T> {
    last_event_count: usize,
    marker: PhantomData<T>,
}

impl<T> EventReader<T> {
    pub fn iter<'a>(&mut self, events: &'a Events<T>) -> impl DoubleEndedIterator<Item=&'a T> {
        self.iter_with_id(events).map(|(event, _id)| event)
    }

    pub fn iter_with_id<'a>(&mut self, events: &'a Events<T>) -> impl DoubleEndedIterator<Item=(&'a T, EventId<T>)> {
        self.iter_internal(events).map(|(event, id)| {
            (event, id)
        })
    }

    fn iter_internal<'a>(&mut self, events: &'a Events<T>) -> impl DoubleEndedIterator<Item=(&'a T, EventId<T>)> {
        // calculate offset
        let a_index = if self.last_event_count > events.a_start_event_count {
            self.last_event_count - events.a_start_event_count
        } else {
            0
        };

        let b_index = if self.last_event_count > events.b_start_event_count {
            self.last_event_count - events.b_start_event_count
        } else {
            0
        };

        self.last_event_count = events.event_count;
        match events.state {
            State::A => events
                .buffer_b
                .get(b_index..)
                .unwrap_or_else(|| &[])
                .iter()
                .map(map_event_with_id)
                .chain(
                    events
                        .buffer_a
                        .get(a_index..)
                        .unwrap_or_else(|| &[])
                        .iter()
                        .map(map_event_with_id),
                ),
            State::B => events
                .buffer_a
                .get(a_index..)
                .unwrap_or_else(|| &[])
                .iter()
                .map(map_event_with_id)
                .chain(
                    events
                        .buffer_b
                        .get(b_index..)
                        .unwrap_or_else(|| &[])
                        .iter()
                        .map(map_event_with_id),
                ),
        }
    }

    pub fn latest<'a>(&mut self, events: &'a Events<T>) -> Option<&'a T> {
        self.latest_with_id(events).map(|(event, _)| event)
    }

    pub fn latest_with_id<'a>(&mut self, events: &'a Events<T>) -> Option<(&'a T, EventId<T>)> {
        self.iter_internal(events).rev().next().map(|(event, id)| {
            (event, id)
        })
    }

    pub fn find_latest<'a>(
        &mut self,
        events: &'a Events<T>,
        predicate: impl FnMut(&&T) -> bool,
    ) -> Option<&'a T> {
        self.find_latest_with_id(events, predicate)
            .map(|(event, _)| event)
    }

    pub fn find_latest_with_id<'a>(
        &mut self,
        events: &'a Events<T>,
        mut predicate: impl FnMut(&&T) -> bool,
    ) -> Option<(&'a T, EventId<T>)> {
        self.iter_internal(events)
            .rev()
            .find(|(event, _id)| predicate(event))
            .map(|(event, id)| {
                (event, id)
            })
    }

    pub fn earliest<'a>(&mut self, events: &'a Events<T>) -> Option<&'a T> {
        self.earliest_with_id(events).map(|(event, _)| event)
    }
    
    pub fn earliest_with_id<'a>(&mut self, events: &'a Events<T>) -> Option<(&'a T, EventId<T>)> {
        self.iter_internal(events).next().map(|(event, id)| {
            (event, id)
        })
    }
}

fn map_event_with_id<T>(event: &Event<T>) -> (&T, EventId<T>) {
    (&event.event, event.id)
}

fn map_event<T>(event: &Event<T>) -> &T {
    &event.event
}

impl<T> Default for EventReader<T> {
    fn default() -> Self {
        Self {
            last_event_count: 0,
            marker: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct SystemExitEvent {
    pub code: u32,
    pub message: String,
}