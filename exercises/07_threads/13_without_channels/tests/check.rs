use ticket_fields::test_helpers::{ticket_description, ticket_title};
use without_channels::data::TicketDraft;
use without_channels::store::TicketStore;

#[test]
fn works() {
    let mut store = TicketStore::new();

    let draft = TicketDraft {
        title: ticket_title(),
        description: ticket_description(),
    };
    let ticket_id1 = store.add_ticket(draft);

    let mut store2 = store.clone();

    let draft = TicketDraft {
        title: ticket_title(),
        description: ticket_description(),
    };
    let ticket_id2 = store2.add_ticket(draft);

    let ticket1 = store.get(ticket_id1).unwrap();
    assert_eq!(ticket_id1, ticket1.read().unwrap().id);

    let ticket2 = store2.get(ticket_id2).unwrap();
    assert_eq!(ticket_id2, ticket2.read().unwrap().id);
}
