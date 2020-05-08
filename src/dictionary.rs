use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

///Loads values form text file to vector line by line (wordlist are suposed to be one word (or character name) per line).
fn read_file_into_vector(file: &Path) -> Vec<String> {
    let f = File::open(file).unwrap();
    let file = BufReader::new(&f);
    let mut vector: Vec<String> = Vec::new();

    for line in file.lines() {
        let l = line.unwrap();
        let word: String = l.trim().parse().expect("Wrong value");
        vector.push(word);
    }
    vector
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum WordType {
    PosNouns,
    NegNouns,
    PosAdj,
    NegAdj,
    PosComp,
    NegComp,
    ValidNames,
    ZeroNouns,
}

#[derive(Debug)]
pub struct Dictionary {
    lexicon: HashMap<WordType, Vec<String>>,
    roman_values: HashMap<char, i32>, 
}

impl Dictionary {
    pub fn build() -> Dictionary {
        let mut dict = Dictionary {
            lexicon: HashMap::new(),
            roman_values: HashMap::new(), 
        };

        let path = Path::new("./lang/");

        dict.lexicon.insert(
            WordType::PosNouns,
            read_file_into_vector(path.join("positive_noun.wordlist").as_path()),
        );
        dict.lexicon.insert(
            WordType::NegNouns,
            read_file_into_vector(path.join("negative_noun.wordlist").as_path()),
        );
        dict.lexicon.insert(
            WordType::PosAdj,
            read_file_into_vector(path.join("positive_adjective.wordlist").as_path()),
        );
        dict.lexicon.insert(
            WordType::NegAdj,
            read_file_into_vector(path.join("negative_adjective.wordlist").as_path()),
        );
        dict.lexicon.insert(
            WordType::PosComp,
            read_file_into_vector(path.join("positive_comparative.wordlist").as_path()),
        );
        dict.lexicon.insert(
            WordType::NegComp,
            read_file_into_vector(path.join("negative_comparative.wordlist").as_path()),
        );
        dict.lexicon.insert(
            WordType::ValidNames,
            read_file_into_vector(path.join("characters.wordlist").as_path()),
        );
        dict.lexicon.insert(
            WordType::ZeroNouns,
            read_file_into_vector(path.join("zero_nouns.wordlist").as_path()),
        );

        dict.roman_values.insert('M', 1000);
        dict.roman_values.insert('D', 500);
        dict.roman_values.insert('C', 100);
        dict.roman_values.insert('L', 50);
        dict.roman_values.insert('X', 10);
        dict.roman_values.insert('V', 5);
        dict.roman_values.insert('I', 1);
        
        dict
    }

    pub fn print(&self) {
        println!("{:#?}", self.lexicon)
    }

    pub fn print_keys(&self) {
        println!("{:#?}", self.lexicon.keys())
    }

    ///Check if word is a noun.
    pub fn is_noun(&self, word: String) -> bool {
        self.lexicon
            .get(&WordType::PosNouns)
            .unwrap()
            .to_vec()
            .iter()
            .any(|v| v == &word)
            || self
                .lexicon
                .get(&WordType::NegNouns)
                .unwrap()
                .to_vec()
                .iter()
                .any(|v| v == &word)
    }

    ///Check if word is an adjective.
    pub fn is_adjective(&self, word: String) -> bool {
        self.lexicon
            .get(&WordType::PosAdj)
            .unwrap()
            .to_vec()
            .iter()
            .any(|v| v == &word)
            || self
                .lexicon
                .get(&WordType::NegAdj)
                .unwrap()
                .to_vec()
                .iter()
                .any(|v| v == &word)
    }

    ///Check if word is a comparative.
    pub fn is_comparative(&self, word: String) -> bool {
        self.lexicon
            .get(&WordType::PosComp)
            .unwrap()
            .to_vec()
            .iter()
            .any(|v| v == &word)
            || self
                .lexicon
                .get(&WordType::NegComp)
                .unwrap()
                .to_vec()
                .iter()
                .any(|v| v == &word)
    }

    ///Returns a value of a noun form a range (-1, 1).
    pub fn noun_value(&self, noun: String) -> i8 {
        if self
            .lexicon
            .get(&WordType::PosNouns)
            .unwrap()
            .to_vec()
            .iter()
            .any(|v| v == &noun)
        {
            1
        } else if self
            .lexicon
            .get(&WordType::NegNouns)
            .unwrap()
            .to_vec()
            .iter()
            .any(|v| v == &noun)
        {
            -1
        } else {
            0
        }
    }

    pub fn is_roman_numeral(&self, roman: char) -> bool {
        self.roman_values.contains_key(&roman)
    }

    pub fn parse_roman_numeral(&self, roman_string: String) -> i32 {

        let chars: Vec<char> = roman_string.to_uppercase().chars().collect();
        let mut string_index = 0;
        let mut roman_sum = 0;

        let mut boo: i32 = 999;

        boo

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}