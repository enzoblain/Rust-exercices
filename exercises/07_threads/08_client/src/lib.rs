use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

#[derive(Clone)]
// TODO: flesh out the client implementation.
pub struct TicketStoreClient {
    inner: Rc<RefCell<TicketStore>>,
}

impl Default for TicketStoreClient {
    fn default() -> Self {
        Self::new()
    }
}

impl TicketStoreClient {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(TicketStore::new())),
        }
    }

    pub fn insert(&self, draft: TicketDraft) -> TicketId {
        self.inner.borrow_mut().add_ticket(draft)
    }

    pub fn get(&self, id: TicketId) -> Option<Ticket> {
        self.inner.borrow().get(id).cloned()
    }
}

pub fn launch() -> TicketStoreClient {
    let (_, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));

    TicketStoreClient::new()
}

#[allow(unused)]
enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
