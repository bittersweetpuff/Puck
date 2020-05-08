mod dictionary;
pub use dictionary::*;

fn main() {}

#[test]
fn word_checks() {
    let dec = Dictionary::build();
    assert_eq!(dec.is_noun("belh".to_string()), false);
    assert_eq!(dec.is_noun("beggar".to_string()), true);
    assert_eq!(dec.is_noun("angel".to_string()), true);

    assert_eq!(dec.is_adjective("belh".to_string()), false);
    assert_eq!(dec.is_adjective("cursed".to_string()), true);
    assert_eq!(dec.is_adjective("golden".to_string()), true);

    assert_eq!(dec.is_comparative("belh".to_string()), false);
    assert_eq!(dec.is_comparative("punier".to_string()), true);
    assert_eq!(dec.is_comparative("fresher".to_string()), true);

    assert_eq!(dec.noun_value("beggar".to_string()), -1);
    assert_eq!(dec.noun_value("angel".to_string()), 1);
    assert_eq!(dec.noun_value("belh".to_string()), 0);
}

#[test]
fn roman_values() {
    let dec = Dictionary::build();
    assert_eq!(dec.get_roman_value('I'), 1);
    assert_eq!(dec.get_roman_value('V'), 5);
    assert_eq!(dec.get_roman_value('X'), 10);
    assert_eq!(dec.get_roman_value('L'), 50);
    assert_eq!(dec.get_roman_value('C'), 100);
    assert_eq!(dec.get_roman_value('D'), 500);
    assert_eq!(dec.get_roman_value('M'), 1000);
}

#[test]
fn roman_parse() {
    let dec = Dictionary::build();
    assert_eq!(dec.parse_roman_numeral("III".to_string()), 3);
    assert_eq!(dec.parse_roman_numeral("II".to_string()), 2);
    assert_eq!(dec.parse_roman_numeral("I".to_string()), 1);
    assert_eq!(dec.parse_roman_numeral("IV".to_string()), 4);
    assert_eq!(dec.parse_roman_numeral("MCMXCIX".to_string()), 1999);
}
