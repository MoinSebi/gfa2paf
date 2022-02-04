use crate::paf::{Paf, Paf_file};
use crate::core::{iterate_test};

mod paf;
mod core;
mod graph2pos;
use clap::{App, Arg};
use gfaR_wrapper::{NGfa, GraphWrapper};


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
        .arg(Arg::new("threads")
            .short('t')
            .long("threads")
            .about("Number of threads")
            .takes_value(true))


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

    let mut threads = 1;
    if matches.is_present("threads"){
        threads = matches.value_of("threads").unwrap().parse().unwrap();
    }

    // Read the graph
    let mut graph:  NGfa = NGfa::new();
    graph.from_graph(gfa);

    // Create the graph wrapper
    let mut graph_wrapper: GraphWrapper = GraphWrapper::new();
    graph_wrapper.fromNGfa(&graph, "_");



    let mut paf_file: Vec<Paf> = Vec::new();
    let mut paf_result = Paf_file::new();
    if matches.is_present("simple") {
        let windows: usize = matches.value_of("simple").unwrap().parse().unwrap();
        iterate_test(&graph, threads, & mut paf_result, &windows);
    }

    paf_result.make_stats();
    paf_result.to_file("test");
}
