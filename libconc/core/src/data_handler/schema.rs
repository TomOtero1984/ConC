use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WordSchema {
    pub index: usize,
    pub natural: String,
    pub conc: String,
}
impl WordSchema {
    pub fn new(index: usize, natural: String, conc: String) -> Self {
        Self {
            index,
            natural,
            conc,
        }
    }
}
