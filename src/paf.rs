use std::collections::HashMap;

/// PAF
pub struct Paf{
    query_name: String,
    query_len: u32,
    query_start: u32,
    query_end: u32,
    strand: bool,
    target_name: String,
    target_len: u32,
    target_start: u32,
    target_end: u32,
    matches_numb: u32,
    alignment_len: u32,
    mapping_qual: u8,
    flag: cg_flag,
    cg_flag: HashMap<u8, String>,

}

impl Paf {
    pub fn new() -> Self {
        Self {
            query_name: "test".to_string(),
            query_len: 10,
            query_start: 10,
            query_end: 10,
            strand: true,
            target_name: "test2".to_string(),
            target_len: 10,
            target_start: 100,
            target_end: 32,
            matches_numb: 32,
            alignment_len: 32,
            mapping_qual: 8,
            flag: cg_flag::new(),
            cg_flag: HashMap::new(),
        }
    }

}

// 1 = match
// 2 = mismatch
// 3 = insertion
// 4 = deletion



pub struct cg_flag{
    flag: Vec<(u8, u32)>,
}

impl cg_flag{
    pub fn new() -> Self {
        Self{
            flag: Vec::new()
        }
    }
}