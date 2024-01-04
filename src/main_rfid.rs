// mod rfid;

use std::error::Error;

fn main() {
    let result : Result<(), Box<dyn Error>>;
    result = Ok(());

    match result {
        Ok(_) => println!("OK, done."),
        Err(e) => println!("Unhandled error! {e}"),
    }
}
