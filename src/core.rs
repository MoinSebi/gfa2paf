use gfaR_wrapper::{NGfa, GraphWrapper, NPath};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use crate::paf::Paf;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::graph2pos::{chunk_inplace, g2p};

/// Helper main function
pub fn main_test(filename: &str){

    // Reading the graph
    let mut graph:  NGfa = NGfa::new();
    graph.from_graph(filename);

    // Create the graph wrapper
    let mut graph_wrapper: GraphWrapper = GraphWrapper::new();
    graph_wrapper.fromNGfa(&graph, "_");



    eprintln!("{}", graph.paths.len());
    eprintln!("{}", graph_wrapper.genomes[0].0);
    eprintln!("ress {}", 10/3);
    eprintln!("Get graph2pos");
    eprintln!("daskd {:?} ", g2p(&graph, 2));





    let gu = iterate_test(&graph, 2);

}


/// Wrapper for paf files
/// multiple function will be added here
/// E.g. iterate_path is the first function
/// Multithreading base function
/// Output are a list of PAFs
pub fn iterate_test(graph: &NGfa, threads: usize){
    eprintln!("Iterate test");

    // Get pairs and
    let pairs = get_all_pairs2(graph);
    let chunks = chunk_inplace(pairs, threads);

    let k = Arc::new(g2p(graph, threads));

    // Resultat
    let mut result = Vec::new();
    let mut rm = Arc::new(Mutex::new(result));
    let mut handles = Vec::new();

    // Iterate over chunks
    for chunk in chunks{
        let r = Arc::clone(&rm);
        let mut r2 = Arc::clone(&k);
        let handle = thread::spawn(move || {
            for pair in chunk.iter(){
                let mut rr = r.lock().unwrap();
                let h = bifurcation_simple(&(&pair.0, &pair.1), &r2);
                rr.push(h);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap()
    }
    eprintln!("{:?}", rm.lock().unwrap());
}


/// Stop the paf when there is more than X "different" sequence
pub fn bifurcation_simple(pair: &(&NPath, &NPath), gfa2pos: &HashMap<String, Vec<usize>>) -> Vec<Paf>{
    let shared = get_shared_direction(pair.0, pair.1);
    let mut paf_vector: Vec<Paf> = Vec::new();
    let shared_vec = get_shared_direction_test(pair.0, pair.1);

    let mut open = false;
    let mut paf_entry = Paf::new();
    let mut last_index = 0;
    let mut last_shared = 0;
    // Inbetweens are for distance calculation
    let mut inbetween1 = Vec::new();
    let mut inbetween2 = Vec::new();

    // Iterate over each pair
    for x in 0..pair.0.nodes.len() {
        // Check if pair is shared
        let node = &(pair.0.nodes[x], pair.0.dir[x]);

        // Wenn shared
        if shared.contains(node) {
            // Iterate over the other path (for the last shared) and check if it is the same
            inbetween2 = Vec::new();
            for x in last_index..pair.1.nodes.len() {

                // If found
                if node == &(pair.1.nodes[x], pair.1.dir[x]) {
                    //eprintln!("hit");
                    // If there is a open paf
                    if inbetween2.len() + inbetween1.len() > 20 {
                        eprintln!("daksljdlka");
                        makepaf();
                        paf_vector.push(paf_entry);
                        paf_entry = Paf::new();
                        paf_entry.query_start = 1;
                        paf_entry.flag.flag.push((1, 20));
                    } else {
                        paf_entry.flag.flag.push((1, 20));
                    }
                    last_index = x.clone();
                    inbetween2 = Vec::new();
                    inbetween1 = Vec::new();
                    break;
                } else {
                    inbetween2.push(x);
                }
            }
        } else {
            inbetween1.push(x);
        }
    }
    paf_vector
}

pub fn makepaf(){
    eprintln!("dsajkdhsajk");
}



// pub fn iterate_path(pair: &(&NPath, &NPath)) -> Vec<(usize, usize)>{
//     let distance = 0;
//     eprintln!("Get shared");
//     let shared = get_shared_direction(pair.0, pair.1);
//     eprintln!("Done");
//     let shared2 = get_shared_direction_test(pair.0, pair.1);
//     let mut last_shared = 0;
//     let k: Vec<Paf> = Vec::new();
//     let mut k1: Vec<(usize, usize)> = Vec::new();
//     let mut indexpairs: (usize, usize) = (0,0);
//     for x in 0..pair.0.nodes.len(){
//         if shared.contains(&(pair.0.nodes[x], pair.0.dir[x])){
//             if (pair.0.nodes[x], pair.0.dir[x]) == shared2.1[last_shared]{
//                 //eprintln!("dajkldhajkshdjka");
//                 last_shared += 1;
//             } else {
//                 //eprintln!("dasjkldjsakldja");
//                 for y in indexpairs.1..pair.1.nodes.len() {
//                     if (&pair.1.nodes[y], &pair.1.dir[y]) == (&pair.0.nodes[x], &pair.0.dir[x]) {
//                         indexpairs = (x.clone(), y.clone());
//                         k1.push(indexpairs);
//                     }
//                 }
//             }
//         }
//     }
//     k1
// }

/// Get all path pairs of a graph
pub fn get_all_pairs(graph: &NGfa) -> Vec<(&NPath, &NPath)> {

    let mut pairs: Vec<(&NPath, &NPath)> = Vec::new();
    for (i1, path1) in graph.paths.iter().enumerate(){
        for path2 in graph.paths[i1+1..].iter(){
            // Optional for checking
            // println!("{} {}", path1.name, path2.name);
            pairs.push((path1, path2));
        }
    }
    pairs
}


/// Get all path pairs of a graph
pub fn get_all_pairs2(graph: &NGfa) -> Vec<(NPath, NPath)> {

    let mut pairs: Vec<(NPath, NPath)> = Vec::new();
    let mut count = 0;
    for path1 in graph.paths.iter(){
        for path2 in graph.paths[count+1..].iter(){
            // Optional for checking
            // println!("{} {}", path1.name, path2.name);
            pairs.push((path1.clone(), path2.clone()));
        }
        count += 1;
    }
    pairs
}

/// Get shared nodes between two paths (with direction correct)
/// Complexity is O(N²)
pub fn get_shared_direction<'a>(test: &'a NPath, test2: &'a NPath) -> HashSet<(u32, bool)>{
    let iter: HashSet<(u32, bool)> = HashSet::from_iter(test.nodes.iter().cloned().zip(test.dir.iter().cloned()));
    let iter2: HashSet<(u32, bool)> = HashSet::from_iter(test2.nodes.iter().cloned().zip(test2.dir.iter().cloned()));

    let g: HashSet<(u32, bool)> = iter.intersection(&iter2).cloned().collect();
    //println!("The length of shared nodes is {}", g.len());
    //println!("Shared nodes {:?}", g);
    g
}



///  For each shared
pub fn get_shared_direction_test<'a>(test: &'a NPath, test2: &'a NPath)
    -> ((Vec<(u32, bool)>, Vec<(u32, bool)>), (Vec<usize>, Vec<usize>)){
    println!("{} {}", test.name, test2.name);
    let i1: Vec<(u32, bool)> = Vec::from_iter(test.nodes.iter().cloned().zip(test.dir.iter().cloned()));
    let i2: Vec<(u32, bool)> = Vec::from_iter(test2.nodes.iter().cloned().zip(test2.dir.iter().cloned()));
    println!("{}", test2.nodes.len());

    let iter: HashSet<(u32, bool)> = HashSet::from_iter(test.nodes.iter().cloned().zip(test.dir.iter().cloned()));
    let iter2: HashSet<(u32, bool)> = HashSet::from_iter(test2.nodes.iter().cloned().zip(test2.dir.iter().cloned()));

    let g: HashSet<(u32, bool)> = iter.intersection(&iter2).cloned().collect();

    let mut shared1 = Vec::new();
    let mut shared1_2 = Vec::new();
    for (index, x) in i1.iter().enumerate(){
        if g.contains(x){
            shared1.push((x.0,x.1));
            shared1_2.push(index);
        }
    }

    let mut shared2 = Vec::new();
    let mut shared2_2 = Vec::new();
    let mut last: &(u32, bool) = &(0,true);
    for (index, x) in i2.iter().enumerate(){
        if g.contains(x){
            if last == x{
                println!("hello hello");
            }
            last = x;
            shared2.push((x.0,x.1));
            shared2_2.push(index);
        }
    }
    // (Vec<u32, bool>, Vec<u32, bool>)
    //println!("The length of shared nodes is {}", g.len());
    //println!("Shared nodes {:?}", g);#
    return ((shared1, shared2), (shared1_2, shared2_2));
}