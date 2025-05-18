
use crate::token::SourceLoc;
use std::fmt::{self, Write};

#[derive(Debug, PartialEq)]
pub enum Key {
    Name(String),
    MacroValue(String),
    MacroSignature { name: String, args: Vec::<String> }
}

impl Key {
    fn pretty_string(&self) -> String {
        match self {
            Key::Name(name) => format!("Name(\"{}\")", name),
            Key::MacroValue(val) => format!("MacroValue({})", val),
            Key::MacroSignature { name, args } => {
                format!(
                    "MacroSignature {{ name: \"{}\", args: {:?} }}",
                    name, args
                )
            }
        }
    }
}

#[derive(Debug, PartialEq)]
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

    pub fn pretty_string(&self) -> String {
        let mut out = String::new();
        let _ = self.pretty_fmt(&mut out, 0);
        out
    }

    fn pretty_fmt(&self, f: &mut String, indent: usize) -> fmt::Result {
        for entry in &self.entries {
            entry.pretty_fmt(f, indent)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
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
    fn pretty_fmt(&self, f: &mut String, indent: usize) -> fmt::Result {
        writeln!(f, "{:indent$}KeyValueEntry {{", "", indent = indent)?;
        writeln!(f, "{:indent$}key: {},", "  ", self.key.pretty_string(), indent = indent + 2)?;
        writeln!(f, "{:indent$}value:", "", indent = indent + 2)?;
        self.value.pretty_fmt(f, indent + 4)?;
        // writeln!(f, "{:indent$}location: {:?},", "", self.location, indent = indent + 2)?;
        writeln!(f, "{:indent$}}}", "", indent = indent)
    }
}

#[derive(Debug, PartialEq)]
pub enum BlockValue {
    Literal(String),
    Expression(String),
    Block(KeyValueBlock),
    MacroValue(String),
    Empty,
}

impl BlockValue {
    fn pretty_fmt(&self, f: &mut String, indent: usize) -> fmt::Result {
        match self {
            BlockValue::Literal(s) => {
                writeln!(f, "{:indent$}Literal(\"{}\"),", "", s, indent = indent)
            }
            BlockValue::Expression(s) => {
                writeln!(f, "{:indent$}Expression(\"{}\"),", "", s, indent = indent)
            }
            BlockValue::MacroValue(s) => {
                writeln!(f, "{:indent$}MacroValue(\"{}\"),", "", s, indent = indent)
            }
            BlockValue::Empty => {
                writeln!(f, "{:indent$}Empty,", "", indent = indent)
            }
            BlockValue::Block(block) => {
                writeln!(f, "{:indent$}Block {{", "", indent = indent)?;
                block.pretty_fmt(f, indent + 2)?;
                writeln!(f, "{:indent$}}},", "", indent = indent)
            }
        }
    }

    fn short_debug(&self) -> String {
        match self {
            BlockValue::Literal(s) => format!("Literal(\"{}\")", s),
            BlockValue::Expression(s) => format!("Expression(\"{}\")", s),
            BlockValue::MacroValue(s) => format!("MacroValue(\"{}\")", s),
            BlockValue::Empty => "Empty".to_string(),
            BlockValue::Block(_) => "Block(...)".to_string(),
        }
    }
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
