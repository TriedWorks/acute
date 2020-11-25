use std::marker::PhantomData;
use acute_ecs::legion::systems::Resource;
use std::fmt;
use futures::SinkExt;

/// Event<T> represents any Event T
pub struct Event<T> {
    pub id: usize,
    pub event: T,
    _marker: PhantomData<T>
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

/// Events<T> is a "buffer" over all events of type T from the last 4 ticks / iterations
/// if an Event<T> is not handled in the last 4 ticks, they are dropped.
#[derive(Debug)]
pub struct Events<T> {
    pub buffer: Vec<Event<T>>,
    pub counter: Vec<usize>,
}

impl<T> Default for Events<T> {
    fn default() -> Self {
        Self {
            buffer: Vec::new(),
            counter: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct SystemExitEvent {
    pub code: u32,
    pub message: String,
}

impl<T: Resource> Events<T> {
    pub fn add(&mut self, event: T) {
        let event =  Event {
            id: self.buffer.len(),
            event,
            _marker: PhantomData,
        };
        self.counter.insert(event.id, 0);
        self.buffer.push(event);
    }

    pub fn update(&mut self) {
        for n in &mut self.counter {
            *n += 1;
        }

        let indices = self.counter.iter()
            .filter(|&&n| n > 3)
            .enumerate().map(|(i, _)| i)
            .collect::<Vec<usize>>();

        indices.iter().for_each(|&i| {
            self.counter.remove(i);
            self.buffer.remove(i);
        });
    }

    pub fn drain(&mut self) -> impl Iterator<Item = T> + '_ {
        self.counter.clear();
        self.buffer.drain(..).map(|e| e.event)
    }

    pub fn clear(&mut self) {
        self.counter.clear();
        self.buffer.clear();
    }
}

#[cfg(test)]
pub mod test {
    use crate::{Events, SystemExitEvent};

    #[test]
    fn test_event_removal() {
        let mut events = Events::<SystemExitEvent>::default();
        assert_eq!(events.buffer.len(), 0);
        // event added: 0
        events.add(SystemExitEvent {code: 1, message: "SystemExitEvent".to_string()});
        assert_eq!(events.buffer.len(), 1);
        assert_eq!(events.counter.len(), 1);
        // update: 1
        events.update();
        assert_eq!(events.buffer.len(), 1);
        assert_eq!(events.counter.len(), 1);
        // update: 2
        events.update();
        assert_eq!(events.buffer.len(), 1);
        assert_eq!(events.counter.len(), 1);
        // event 2 added: 2 | 0
        events.add(SystemExitEvent {code: 0, message: "Default Event".to_string()});
        assert_eq!(events.buffer.len(), 2);
        assert_eq!(events.counter.len(), 2);
        // update: 3 | 1
        events.update();
        assert_eq!(events.buffer.len(), 2);
        assert_eq!(events.counter.len(), 2);
        // update: 4 | 2 => remove first event
        events.update();
        assert_eq!(events.buffer.len(), 1);
        assert_eq!(events.counter.len(), 1);
        // update: | 3 => only second event is there
        events.update();
        assert_eq!(events.buffer.len(), 1);
        assert_eq!(events.counter.len(), 1);

    }
}