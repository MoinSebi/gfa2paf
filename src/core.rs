use gfaR_wrapper::{NGfa, GraphWrapper, NPath};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use crate::paf::Paf;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn main_test(filename: &str){

    // Reading the graph
    let mut graph:  NGfa = NGfa::new();
    graph.from_graph(filename);
    let mut graph_wrapper: GraphWrapper = GraphWrapper::new();
    graph_wrapper.fromNGfa(&graph, "_");
    eprintln!("{}", graph.paths.len());
    eprintln!("{}", graph_wrapper.genomes[0].0);
    eprintln!("ress {}", 10/3);
    eprintln!("Get graph2pos");
    eprintln!("daskd {:?} ", g2p(&graph, 2));
    iterate_test(&graph);



    //iterate_test(&graph);

}


pub fn chunk_inplace<T>(it: Vec<T>, numb: usize) -> Vec<Vec<T>>{
    let mut vec_new: Vec<Vec<T>> = Vec::new();
    for x in 0..numb{
        vec_new.push(Vec::new());
    }
    let each_size = (it.len() as f64 /numb as f64).ceil() as usize;
    eprintln!("Number {}", each_size);

    let mut count = 0;
    for x in it{

        vec_new[count/numb].push(x);

    }
    vec_new

}

pub fn g2p(graph: & gfaR_wrapper::NGfa, threads: usize) {

    let mut result_hm: HashMap<String, Vec<usize>> = HashMap::new();
    let mut result = Arc::new(Mutex::new(result_hm));
    let mut hm = Arc::new(graph.nodes.clone());
    let k = graph.paths.clone();
    let k2 = chunk_inplace(k, threads);
    let mut handles: Vec<_> = Vec::new();
    println!("sda das {}", k2.len());
    for chunk in k2{
        let mut g2 = Arc::clone(&hm);
        let mut tess1 = Arc::clone(&result);
        let handle = thread::spawn(move || {
            eprintln!("I spawned");
            for c in chunk{
                let mut position = 0;
                let mut vec_pos: Vec<usize> = Vec::new();
                for y in c.nodes.iter(){
                    position += g2.get(y).unwrap().seq.len();
                    vec_pos.push(position);
                }
                let mut lo = tess1.lock().unwrap();
                lo.insert(c.name.clone(), vec_pos);


            }
            eprintln!("Im done");
        });
        handles.push(handle);
        eprintln!("hello");
    }

    let mut count = 0;
    for handle in handles {
        eprintln!("{}", count);
        count += 1;
        handle.join().unwrap()
    }



}

/// Convert index in the graph to positional information
/// Index based - not node based
/// [10,30,31,32,45]
// pub fn graph2pos(graph: & gfaR_wrapper::NGfa) -> HashMap<String, Vec<usize>>{
//     let mut result_hm: HashMap<String, Vec<usize>> = HashMap::new();
//     let result = Arc::new(Mutex::new(HashMap::new()));
//
//
//     let mut handles: Vec<_> = Vec::new();
//     let chunks = graph.paths.chunks(graph.paths.len()/3);
//     let mut t2= Arc::new(graph.paths.clone());
//     eprintln!("{}", t2.len());
//     for chunk in 0..4 {
//         let mut tess1 = Arc::clone(&result);
//         let mut t3 = Arc::clone(&t2);
//         eprintln!("lel {}", mutt2.len());
//         let handle = thread::spawn(move || {
//             //let t = t3.len();
//             let mut g = tess1.lock().unwrap();
//             g.insert("10", chunk);
//         });
//             handles.push(handle);
//             // for x in graph.paths.iter
//             // let mut vec_pos: Vec<usize> = Vec::new();
//             // let mut position: usize = 0;
//             // for y in x.nodes.iter(){
//             //     position += graph.nodes.get(y).unwrap().seq.len();
//             //     vec_pos.push(position);
//             // }
//             // result_hm.insert(x.name.clone(), vec_pos);
//
//     }
//
//     // wait for each thread to finish
//     for handle in handles {
//         handle.join().unwrap()
//     }
//     println!("{:?}", result);
//     result_hm
// }

pub fn iterate_test(graph: &NGfa){
    eprintln!("Iterate test");
    let pairs = get_all_pairs(graph);

    let g2 = graph.clone();
    let pairs = get_all_pairs2(graph);
    let chunks = chunk_inplace(pairs, 4);
    let mut handles = Vec::new();
    let mut last_shared = 0;

    for chunk in chunks{
        let handle = thread::spawn(move || {
            for pair in chunk.iter(){
                iterate_path(&(&pair.0, &pair.1));
            }
        });
        handles.push(handle);
    }

    let mut count = 0;
    for handle in handles {
        eprintln!("{}", count);
        count += 1;
        handle.join().unwrap()
    }
}

pub fn iterate_path(pair: &(&NPath, &NPath)) -> Vec<(usize, usize)>{
    let distance = 0;
    eprintln!("Get shared");
    let shared = get_shared_direction(pair.0, pair.1);
    eprintln!("Done");
    let shared2 = get_shared_direction_test(pair.0, pair.1);
    let mut last_shared = 0;
    let k: Vec<Paf> = Vec::new();
    let mut k1: Vec<(usize, usize)> = Vec::new();
    let mut indexpairs: (usize, usize) = (0,0);
    for x in 0..pair.0.nodes.len(){
        if shared.contains(&(pair.0.nodes[x], pair.0.dir[x])){
            if (pair.0.nodes[x], pair.0.dir[x]) == shared2.1[last_shared]{
                //eprintln!("dajkldhajkshdjka");
                last_shared += 1;
            } else {
                //eprintln!("dasjkldjsakldja");
                for y in indexpairs.1..pair.1.nodes.len() {
                    if (&pair.1.nodes[y], &pair.1.dir[y]) == (&pair.0.nodes[x], &pair.0.dir[x]) {
                        indexpairs = (x.clone(), y.clone());
                        k1.push(indexpairs);
                    }
                }
            }
        }
    }
    k1
}

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

pub fn get_shared_direction_test<'a>(test: &'a NPath, test2: &'a NPath) -> (Vec<(u32, bool)>, Vec<(u32, bool)>){
    println!("{} {}", test.name, test2.name);
    let i1: Vec<(u32, bool)> = Vec::from_iter(test.nodes.iter().cloned().zip(test.dir.iter().cloned()));
    let i2: Vec<(u32, bool)> = Vec::from_iter(test2.nodes.iter().cloned().zip(test2.dir.iter().cloned()));
    println!("{}", test2.nodes.len());

    let iter: HashSet<(u32, bool)> = HashSet::from_iter(test.nodes.iter().cloned().zip(test.dir.iter().cloned()));
    let iter2: HashSet<(u32, bool)> = HashSet::from_iter(test2.nodes.iter().cloned().zip(test2.dir.iter().cloned()));

    let g: HashSet<(u32, bool)> = iter.intersection(&iter2).cloned().collect();

    let mut shared1 = Vec::new();
    for x in i1.iter(){
        if g.contains(x){
            shared1.push(x.clone());
        }
    }

    let mut shared2 = Vec::new();
    let mut last: &(u32, bool) = &(0,true);
    for x in i2.iter(){
        if g.contains(x){
            if last == x{
                println!("hello hello");
            }
            last = x;
            shared2.push(x.clone());
        }
    }
    eprintln!("{}", g.len());
    eprintln!("{}", shared1.len());
    eprintln!("{}", shared2.len());
    eprintln!("{}", i1.len());
    eprintln!("{}", iter.len());
    eprintln!("{}", iter2.len());
    if shared1.len() == iter.len(){
        eprintln!("{}", shared1.len());
        eprintln!("{}", iter.len());
        eprintln!("dajkdhsajkd");
    }
    // (Vec<u32, bool>, Vec<u32, bool>)
    //println!("The length of shared nodes is {}", g.len());
    //println!("Shared nodes {:?}", g);#
    return (shared1, shared2)
}