use std::collections::HashMap;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;


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
}

impl Dictionary {
    pub fn build() -> Dictionary {

        let mut dict = Dictionary {
        lexicon: HashMap::new(),
        };

        let path = Path::new("./lang/");

        dict.lexicon.insert(WordType::PosNouns, read_file_into_vector(path.join("positive_noun.wordlist").as_path()));
        dict.lexicon.insert(WordType::NegNouns, read_file_into_vector(path.join("negative_noun.wordlist").as_path()));
        dict.lexicon.insert(WordType::PosAdj, read_file_into_vector(path.join("positive_adjective.wordlist").as_path()));
        dict.lexicon.insert(WordType::NegAdj, read_file_into_vector(path.join("negative_adjective.wordlist").as_path()));
        dict.lexicon.insert(WordType::PosComp, read_file_into_vector(path.join("positive_comparative.wordlist").as_path()));
        dict.lexicon.insert(WordType::NegComp, read_file_into_vector(path.join("negative_comparative.wordlist").as_path()));
        dict.lexicon.insert(WordType::ValidNames, read_file_into_vector(path.join("characters.wordlist").as_path()));
        dict.lexicon.insert(WordType::ZeroNouns, read_file_into_vector(path.join("zero_nouns.wordlist").as_path()));

        dict
        
    }

    pub fn print(&self) {
        println!("{:#?}", self.lexicon)
    }

    pub fn print_keys(&self) {
        println!("{:#?}", self.lexicon.keys())
    }
}