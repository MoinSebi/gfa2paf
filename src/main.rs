use crate::paf::Paf;
use crate::core::{main_test};

mod paf;
mod core;
mod graph2pos;
use clap::{App, Arg};


fn main() {
    eprintln!("Running gfa2paf");
    let matches = App::new("panSV")
        .version("0.1.0")
        .author("Sebastian V")
        .about("gfa2bin")
        // Input
        .arg(Arg::new("gfa")
            .short('g')
            .long("gfa")
            .about("Sets the input file to use")
            .takes_value(true)
            .required(true))


        .arg(Arg::new("simple")
            .short('s')
            .long("simple")
            .about("Simple algorithm")
            .takes_value(true))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .about("output file")
            .takes_value(true)).get_matches();


    let gfa = matches.value_of("gfa").unwrap();
    let old = "/home/svorbrugg_local/panSV/graphs/testGraph.gfa";
    main_test(gfa);
    println!("Hello, world!");
}
