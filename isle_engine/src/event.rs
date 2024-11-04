use std::{iter, sync::{Arc, Mutex, OnceLock}};

type NodeReference<T> = Arc<OnceLock<EventNode<T>>>;

pub trait EventArgs: Clone {}

#[derive(Clone)]
struct EventNode<T: EventArgs> {
    event: T,
    next: NodeReference<T>
}

impl<T: EventArgs> EventNode<T> {
    fn new(event: T) -> Self {
        Self {
            event,
            next: Arc::new(OnceLock::new())
        }
    }

    pub fn push(&self, event: T) -> NodeReference<T> {
        let result = if let Some(next) = self.next.get() {
            Ok(next.push(event.clone()))
        } else {
            self.next
                .set(Self::new(event.clone()))
                .map(|_| self.next.clone())
        };

        if let Ok(node) = result {
            node
        } else {
            self.push(event)
        }
    }
}

#[derive(Clone)]
pub struct EventWriter<T: EventArgs> {
    last: Arc<Mutex<NodeReference<T>>>,
}

impl<T: EventArgs> EventWriter<T> {
    pub fn new() -> Self {
        Self {
            last: Default::default(),
        }
    }

    pub fn send(&mut self, event: T) {
        let mut last = self.last.lock().unwrap();
        let _ = if let Some(last_evt) = last.get() {
            *last = last_evt.push(event.clone());
            Ok(())
        } else {
            last.set(EventNode::new(event.clone()))
        };
    }

    fn last(&self) -> NodeReference<T> {
        self.last.lock().unwrap().clone()
    }
}

#[derive(Clone)]
pub struct EventReader<T: EventArgs> {
    head: NodeReference<T>,
}

impl<T: EventArgs> EventReader<T> {
    pub fn from_writer(writer: &EventWriter<T>) -> Self {
        Self {
            head: writer.last().clone(),
        }
    }

    pub fn read(&mut self) -> Option<T> {
        self.head.get().cloned().map(|node| {
            self.head = node.next;
            node.event
        })
    }

    pub fn iter(&mut self) -> impl Iterator<Item = T> + '_ {
        iter::from_fn(move || {
            self.head.get().cloned().map(|node| {
                self.head = node.next.clone();
                node.event
            })
        })
    }
}