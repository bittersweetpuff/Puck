use std::collections::HashMap;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;


fn read_file_into_vector(filename: String) -> Vec<String> {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    let mut vector: Vec<String> = Vec::new();

    for line in file.lines() {
        let l = line.unwrap();
        let word: String = l.trim().parse().expect("Błędna wartość");
        vector.push(word);
    }
    vector
}

#[derive(Debug)]
pub struct Dictionary {
    lexicon: HashMap<String, Vec<String>>,   
}

impl Dictionary {
    pub fn build() -> Dictionary {

        let mut dict = Dictionary {
        lexicon: HashMap::new(),
        };

        dict.lexicon.insert("Characters".to_string(), read_file_into_vector("language/test.wordlist".to_string()));

        dict
        
    }

    pub fn print(&self) {
        println!("{:#?}", self.lexicon)
    }
}