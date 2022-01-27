use crate::paf::Paf;
use crate::core::{main_test};

mod paf;
mod core;
mod graph2pos;

fn main() {
    let paf = Paf::new();
    main_test("/home/svorbrugg_local/Rust/data/AAA_AAB.cat.gfa");
    println!("Hello, world!");
}
