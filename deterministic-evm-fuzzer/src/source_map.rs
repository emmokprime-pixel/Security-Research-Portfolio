use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceLocation {
    pub offset: usize,
    pub length: usize,
    pub file_id: i32,
    pub jump: char,
}

pub struct SourceMap {
    pub mappings: Vec<Option<SourceLocation>>,
}

impl SourceMap {
    pub fn decode(map_str: &str, bytecode: &[u8]) -> Self {
        let mut mappings = vec![None; bytecode.len()];
        let mut last_loc = SourceLocation {
            offset: 0,
            length: 0,
            file_id: -1,
            jump: '-',
        };

        for (i, part) in map_str.split(';').enumerate() {
            if i >= bytecode.len() { break; }
            
            let fields: Vec<&str> = part.split(':').collect();
            if !fields.get(0).unwrap_or(&"").is_empty() {
                last_loc.offset = fields[0].parse().unwrap_or(last_loc.offset);
            }
            if fields.get(1).map_or(false, |s| !s.is_empty()) {
                last_loc.length = fields[1].parse().unwrap_or(last_loc.length);
            }
            if fields.get(2).map_or(false, |s| !s.is_empty()) {
                last_loc.file_id = fields[2].parse().unwrap_or(last_loc.file_id);
            }
            if fields.get(3).map_or(false, |s| !s.is_empty()) {
                last_loc.jump = fields[3].chars().next().unwrap_or(last_loc.jump);
            }

            mappings[i] = Some(last_loc.clone());
        }

        Self { mappings }
    }

    pub fn get_location(&self, pc: usize) -> Option<&SourceLocation> {
        self.mappings.get(pc)?.as_ref()
    }
}
