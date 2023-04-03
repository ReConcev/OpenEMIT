#![allow(dead_code)]
struct Ledger {
    name: String,
    scope: Scope,
    persist: bool,
    storage_provider: Provider,
    pub_provider: Provider,
    sub_provider: Provider,
}
impl Ledger {
    fn new(
        name: String,
        scope: Scope,
        persist: bool,
        storage_provider: Provider,
        pub_provider: Provider,
        sub_provider: Provider,
    ) -> Self {
        Self {
            name,
            scope,
            persist,
            storage_provider,
            pub_provider,
            sub_provider,
        }
    }
}
enum Scope {
    Host,
}
type Provider = String;

type Label = [char;30];
struct Topic<'a> {
    name: String,
    parent: &'a Topic<'a>,
}
//static
impl Topic <'_> {
    fn new(name: String, parent: &Topic) -> Self {
        Self {
            name,
            parent,
        }
    }
}
trait Publish {
    fn publish() {
        println!("Event Published on Topic");
    }
}
trait Subscribe {
    fn subscribe() {
        println!("Topic Subscribed");
    }
}
impl Publish for Topic {}

impl Subscribe for Topic {}

struct Event {
    emit_header: EmitHeader,
    event_header: EventHeader,
    private_data: PrivateData,
    public_data: PublicData,
}
struct EmitHeader;
struct EventHeader;
struct PrivateData;
struct PublicData;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ledger = Ledger::new("TestLedger".to_owned(), Scope::Host, "MinIO".to_owned());
        let topic = Topic::new("test".to_owned(), None, ledger);
        assert_eq!(ledger.provider, "MinIO".to_owned());
    }
}
