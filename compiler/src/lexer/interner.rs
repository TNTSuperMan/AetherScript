use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LiteralId(usize);

pub struct LiteralInterner {
    literal_map: HashMap<String, LiteralId>,
}

impl LiteralInterner {
    pub fn new() -> Self {
        Self {
            literal_map: HashMap::new(),
        }
    }
    pub fn get_or_insert(&mut self, literal: String) -> LiteralId {
        let next_id = LiteralId(self.literal_map.len());
        *self.literal_map.entry(literal).or_insert(next_id)
    }
    pub fn get_id(&self, literal: &String) -> Option<LiteralId> {
        self.literal_map.get(literal).copied()
    }
    pub fn get_literal(&self, id: LiteralId) -> Option<&str> {
        self.literal_map
            .iter()
            .find(|(_, v)| **v == id)
            .map(|(k, _)| k.as_str())
    }
}
