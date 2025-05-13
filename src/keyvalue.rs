
use crate::token::SourceLoc;


pub type Key = String;

#[derive(Debug)]
pub struct KeyValueBlock {
    pub entries: Vec<KeyValueEntry>
}

impl KeyValueBlock {
    pub fn new() -> Self {
        Self {
            entries: vec![]
        }
    }
    pub fn add(&mut self, entry: KeyValueEntry) {
        self.entries.push(entry);
    }

    pub fn get(&self, key: Key) -> Option<&BlockValue> {
        self.entries.iter()
            .find_map(|e| (e.key == key).then_some(&e.value))
    }
}

#[derive(Debug)]
pub struct KeyValueEntry {
    key: Key,
    value: BlockValue,
    location: SourceLoc,
}

impl KeyValueEntry {
    pub fn new(key: Key, location: SourceLoc, value: BlockValue) -> Self {
            Self {
                key,
                location,
                value,
            }
        }
    }

#[derive(Debug)]
pub enum BlockValue {
    Literal(String),
    Expression(String),
    Block(KeyValueBlock),
    Empty,
}

#[cfg(test)]
mod tests {
    use super::KeyValueBlock;

    #[test]
    fn blocks() {
        let _ = KeyValueBlock {
            entries: vec![]
        };
    }
}
