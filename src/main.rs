mod dictionary;
pub use dictionary::*;

fn main() {

    let dec = Dictionary::build();
    dec.print();
    dec.print_keys();

}
