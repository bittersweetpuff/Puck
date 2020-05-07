mod dictionary;
pub use dictionary::*;

fn main() {
    let dec = Dictionary::build();

    println!("{}", dec.is_noun("belh".to_string()));
    println!("{}", dec.is_noun("beggar".to_string()));
    println!("{}", dec.is_noun("angel".to_string()));

    println!("{}", dec.is_adjective("belh".to_string()));
    println!("{}", dec.is_adjective("cursed".to_string()));
    println!("{}", dec.is_adjective("golden".to_string()));

    println!("{}", dec.is_comparative("belh".to_string()));
    println!("{}", dec.is_comparative("punier".to_string()));
    println!("{}", dec.is_comparative("fresher".to_string()));

    println!("{}", dec.noun_value("beggar".to_string()));
    println!("{}", dec.noun_value("angel".to_string()));
    println!("{}", dec.noun_value("belh".to_string()));
}
