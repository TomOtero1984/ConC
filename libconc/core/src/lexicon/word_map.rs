use crate::lexicon::utils::convert_to_base4096;
use std::collections::HashMap;
use std::vec::Vec;

pub type ConcSymbol = String;
pub type ConcSymbolSet = Vec<ConcSymbol>;

pub type Index = usize;
pub type ConcChar = (Index, ConcSymbol);
pub type ConcCharHash = HashMap<ConcSymbol, ConcChar>;

pub struct CharMap {
    conc_char_set: ConcSymbolSet,
    conc_char_hash: ConcCharHash,
}
impl CharMap {
    pub fn new() -> Self {
        Self {
            conc_char_set: ConcSymbolSet::new(),
            conc_char_hash: ConcCharHash::new(),
        }
    }

    pub fn get_symbol_index(&self, conc_symbol: &ConcSymbol) -> Option<Index> {
        self.conc_char_hash.get(conc_symbol).map(|x| x.0)
    }
    pub fn get_symbol(&self, symbol_index: Index) -> Option<ConcSymbol> {
        self.conc_char_set.get(symbol_index as usize).cloned()
    }
}

pub enum WordType {
    Conc,
    Natural,
}

pub struct Word {
    pub index: Index,
    pub text: String,
    pub language: WordType,
}

pub type ConcWord = Word;
pub type NaturalWord = Word;


pub type ConcHash = HashMap<String, Index>;
pub type NaturalHash = HashMap<String, Index>;

pub struct WordEntry {
    pub natural: NaturalWord,
    pub conc: ConcWord,
}
impl WordEntry {
    pub fn new(natural: NaturalWord, conc: ConcWord) -> Self{
        Self {
            natural,
            conc,
        }
    }
}
pub type WordHash = HashMap<Index, WordEntry>;

pub struct WordMap {
    pub conc_hash: ConcHash,
    pub natural_hash: NaturalHash,
    pub word_count: usize,
    pub word_hash: WordHash
}
impl WordMap {
    pub fn new() -> Self {
        Self {
            conc_hash: ConcHash::new(),
            natural_hash: NaturalHash::new(),
            word_count: 0,
            word_hash: WordHash::new(),
        }
    }

    pub fn generate(&mut self, word_list: &[String]) {
        for (index, word) in word_list.iter().enumerate() {
            let index = index as Index;
            let conc_word = ConcWord {
                index,
                text: convert_to_base4096(index),
                language: WordType::Conc,
            };
            let natural_word = NaturalWord {
                index,
                text: word.clone(),
                language: WordType::Natural,
            };
            self.conc_hash.insert(conc_word.text.clone(), index);
            self.natural_hash.insert(natural_word.text.clone(), index);
            let entry = WordEntry::new(natural_word, conc_word);
            self.word_hash.insert(index, entry);
        }
        self.word_count = word_list.len();
    }
    pub fn get_conc_word(&self, index: Index) -> Option<&ConcWord> {
        self.word_hash.get(&index).map(|x| &x.conc)
    }
    pub fn get_natural_word(&self, index: Index) -> Option<&NaturalWord> {
        self.word_hash.get(&index).map(|x| &x.natural)
    }

    pub fn get_index_by_text(&self, text: &str, word_type: WordType) -> Option<Index> {
        match word_type {
            WordType::Conc => self.conc_hash.get(text).cloned(),
            WordType::Natural => self.natural_hash.get(text).cloned(),
        }
    }
}
