use std::collections::HashMap;

#[derive(Debug)]
/// PAF
pub struct Paf{
    pub query_name: String,
    pub query_len: u32,
    pub query_start: u32,
    pub query_end: u32,
    pub strand: bool,
    pub target_name: String,
    pub target_len: u32,
    pub target_start: u32,
    pub target_end: u32,
    pub matches_numb: u32,
    pub alignment_len: u32,
    pub mapping_qual: u8,
    pub flag: cg_flag,
    pub cg_flag: HashMap<u8, String>,

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


#[derive(Debug)]
pub struct cg_flag{
    pub flag: Vec<(u8, u32)>,
}

impl cg_flag{
    pub fn new() -> Self {
        Self{
            flag: Vec::new()
        }
    }
}