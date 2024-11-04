use std::{iter, sync::{Arc, Mutex, OnceLock}};

type NodeReference<T> = Arc<OnceLock<EventNode<T>>>;

pub trait EventArgs: Clone + std::fmt::Debug {}

#[derive(Clone, Debug)]
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
        last.set(EventNode::new(event)).expect("Last node in invalid state");
        *last = last.get().unwrap().next.clone();
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

#[cfg(test)]
mod test {
    use std::thread;

    use super::*;

    #[derive(Clone, Debug)]
    struct Event(usize);

    impl EventArgs for Event {}

    fn make_channel() -> (EventWriter<Event>, EventReader<Event>) {
        let mut writer = EventWriter::<Event>::new();
        let reader = EventReader::from_writer(&writer);

        (0..5).map(Event).for_each(|event| writer.send(event));

        (writer, reader)
    }

    #[test]
    fn test_write_read() {
        let (writer, mut reader) = make_channel();

        let head_ref = Arc::downgrade(&reader.head);

        let events: Vec<_> = reader.iter().map(|Event(i)| i).collect();
        let comp: Vec<usize> = (0..5).collect();

        assert_eq!(events, comp);
        assert!(Arc::ptr_eq(&writer.last(), &reader.head));
        assert!(head_ref.upgrade().is_none());
        assert!(reader.head.get().is_none());
    }

    #[test]
    fn test_mt_read() {
        let (writer, mut reader) = make_channel();
        let head_ref = Arc::downgrade(&reader.head);

        let mut reader_clone = reader.clone();
        let writer_clone = writer.clone();

        let thread = thread::spawn(move || {
            reader_clone.iter().for_each(|_| {});
            assert!(Arc::ptr_eq(&writer_clone.last(), &reader_clone.head));
        });

        let head_deref = head_ref.upgrade().unwrap();
        assert!(Arc::ptr_eq(&reader.head, &head_deref));
        let Event(i) = reader.read().unwrap();
        assert_eq!(i, 0);
        drop(head_deref);
        thread.join().unwrap();
        assert!(head_ref.upgrade().is_none());
    }
}