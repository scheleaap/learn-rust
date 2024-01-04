mod rfid;

fn main() {
    match rfid::rfid() {
        Ok(_) => println!("OK, done."),
        Err(e) => println!("Unhandled error! {e}"),
    }
}
