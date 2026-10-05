use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentifierId(usize);

pub struct IdentifierInterner {
    literal_map: HashMap<String, IdentifierId>,
}

impl IdentifierInterner {
    pub fn new() -> Self {
        Self {
            literal_map: HashMap::new(),
        }
    }
    pub fn get_or_insert(&mut self, literal: String) -> IdentifierId {
        let next_id = IdentifierId(self.literal_map.len());
        *self.literal_map.entry(literal).or_insert(next_id)
    }
    pub fn get_id(&self, literal: &String) -> Option<IdentifierId> {
        self.literal_map.get(literal).copied()
    }
    pub fn get_name(&self, id: IdentifierId) -> Option<&str> {
        self.literal_map
            .iter()
            .find(|(_, v)| **v == id)
            .map(|(k, _)| k.as_str())
    }
}
