use gfaR_wrapper::{NGfa, NPath, NNode};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use crate::paf::{Paf, PafFile};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::graph2pos::{chunk_inplace, g2p};
use std::cmp::min;
use crate::simple::bifurcation_simple;


/// Wrapper for paf files
/// multiple function will be added here
/// E.g. iterate_path is the first function
/// Multithreading base function
/// Output are a list of PAFs
pub fn iterate_test(graph: &NGfa, threads: usize, paffile: &mut PafFile, maxdistance: &usize) {
    // Get pairs and
    let pairs = get_all_pairs2(graph);
    let chunks = chunk_inplace(pairs, threads);

    let k = Arc::new(g2p(graph, threads));
    let k2 = Arc::new(graph.nodes.clone());

    // Resultat
    let result = Vec::new();
    let rm = Arc::new(Mutex::new(result));
    let mut handles = Vec::new();
    let dist = Arc::new(maxdistance.clone());


    // Iterate over chunks
    for chunk in chunks{
        let r = Arc::clone(&rm);
        let r2 = Arc::clone(&k);
        let r3 = Arc::clone(&k2);
        let ko = Arc::clone(&dist);
        let handle = thread::spawn(move || {
            for pair in chunk.iter(){
                eprintln!("Working on this pair: {} {}", pair.0.name, pair.1.name);
                let h = bifurcation_simple(&(&pair.0, &pair.1), &r2, &r3, **&ko);
                let mut rr = r.lock().unwrap();
                rr.extend(h);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap()

    }
    //eprintln!("{:?}", rm.lock().unwrap());

    let i = rm.lock().unwrap().clone();
    for x in i{
        paffile.paf_entries.push(x);
    }
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
    let i1: Vec<(u32, bool)> = Vec::from_iter(test.nodes.iter().cloned().zip(test.dir.iter().cloned()));
    let i2: Vec<(u32, bool)> = Vec::from_iter(test2.nodes.iter().cloned().zip(test2.dir.iter().cloned()));


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
    for (index, x) in i2.iter().enumerate(){
        if g.contains(x){
            shared2.push((x.0,x.1));
            shared2_2.push(index);
        }
    }
    // (Vec<u32, bool>, Vec<u32, bool>)
    //println!("The length of shared nodes is {}", g.len());
    //println!("Shared nodes {:?}", g);#
    return ((shared1, shared2), (shared1_2, shared2_2));
}