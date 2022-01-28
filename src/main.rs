use crate::paf::Paf;
use crate::core::{main_test};

mod paf;
mod core;
mod graph2pos;

fn main() {
    main_test("/home/svorbrugg_local/panSV/graphs/testGraph.gfa");
    println!("Hello, world!");
}
