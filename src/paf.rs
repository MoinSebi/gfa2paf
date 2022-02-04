use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufWriter};


#[derive(Debug, Clone)]
/// PAF file
pub struct Paf_file{
    pub paf_entries: Vec<Paf>,
    paf_len: usize,
    paf_flags: HashMap<u8, String>,
}

impl Paf_file{
    pub fn new() -> Self{
        Self{
            paf_entries: Vec::new(),
            paf_len: 0,
            paf_flags: HashMap::new(),
        }
    }

    pub fn make_stats(self: &mut Self){
        for x in 0..self.paf_entries.len(){
            self.paf_entries[x].matches();
            self.paf_entries[x].alignment_length();
            self.paf_entries[x].merging();
        }
    }

    pub fn to_file(self: & mut Self, filename: &str){
        let file =  File::create(filename).expect("Unable to create file");
        let mut file = BufWriter::new(file);

        for x in 0..self.paf_entries.len(){
            let p = self.paf_entries[x].printall();
            write!(file, "{}", p).expect("Not working");
        }
    }

}



#[derive(Debug, Clone)]
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
    pub fn new(s1: &String, s2: &String, qstart: &u32, tstart: &u32, qlen: &u32, tlen: &u32) -> Self {
        Self {
            query_name: s1.clone(),
            query_len: qlen.clone(),
            query_start: qstart.clone(),
            query_end: 10,
            strand: true,
            target_name: s2.clone(),
            target_len: tlen.clone(),
            target_start: tstart.clone(),
            target_end: 32,
            matches_numb: 32,
            alignment_len: 32,
            mapping_qual: 255,
            flag: cg_flag::new(),
            cg_flag: HashMap::new(),
        }
    }

    pub fn merging(self: & mut Self){
        let mut vec_new: Vec<(u32, u32)> = Vec::new();

        let mut element: (u32, u32)  = (0,0);
        for x in 0..self.flag.flag.len(){

            if element.0 == self.flag.flag[x].0{
                element.1 += self.flag.flag[x].1.clone();
            } else {
                vec_new.push(element);
                element = self.flag.flag[x].clone();
            }
        }
        vec_new.push(element);
        vec_new.remove(0);
        self.flag.flag = vec_new;
    }

    pub fn printing(self: &Self) -> String{
        let mut s = "".to_string();
        for x in self.flag.flag.iter(){
            if x.0 == 1{
                s.push_str(&x.1.clone().to_string());
                s.push_str("=");
            } else if x.0 == 2{
                s.push_str(&x.1.clone().to_string());
                s.push_str("I");
            } else if x.0 == 3 {

                s.push_str(&x.1.clone().to_string());
                s.push_str("D");
            } else {

                s.push_str(&x.1.clone().to_string());
                s.push_str("X");
            }
        }
        s

    }

    pub fn matches(self: & mut Self){
        let mut count = 0;
        for x in self.flag.flag.iter(){
            if x.0 == 1{
                count += x.1;
            }
        }
        self.matches_numb = count;
    }

    pub fn alignment_length(self: &mut Self){
        let mut lenns = 0;
        for x in self.flag.flag.iter(){
            lenns += x.1;

        }
        self.alignment_len = lenns;
    }

    pub fn printall(self: & mut Self) -> String {

        let output = format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\tcg:Z:{}\n",
                             self.query_name,
                             self.query_len,
                             self.query_start,
                             self.query_end,
                             helpdir(&self.strand),
                             self.target_name,
                             self.target_len,
                             self.target_start,
                             self.target_end,
                             self.matches_numb,
                             self.alignment_len,
                             self.mapping_qual,
                             self.printing());
        output
    }

}

pub fn helpdir(b: &bool) -> &str{
    if b.clone(){
        "+"
    } else {
        "-"
    }
}

// 1 = match
// 2 = mismatch
// 3 = insertion
// 4 = deletion


#[derive(Debug, Clone)]
pub struct cg_flag{
    pub flag: Vec<(u32, u32)>,
}

impl cg_flag{
    pub fn new() -> Self {
        Self{
            flag: Vec::new()
        }
    }
}