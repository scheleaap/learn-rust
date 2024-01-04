fn main() {
    println!("Hello, and welcome to the Rust-based RFID Jukebox!");

    let cards = vec![
        Card { uid: "1", typ: "fake" },
        Card { uid: "2", typ: "fake" },
        Card { uid: "3", typ: "fake" },
    ];

    cards
        .iter()
        .for_each(|c| println!("Read card of type \"{}\" with UID {}", c.typ, c.uid));

    println!("Goodbye, for now. 😢")
}

#[derive(Debug)]
struct Card<'a> {
    uid: &'a str,
    typ: &'a str,
}

trait CardReader {
    // fn read(&self) -> Iter<Option<Card>>;
}