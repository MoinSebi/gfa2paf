use gfaR_wrapper::{NGfa, GraphWrapper, NPath, NNode};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use crate::paf::Paf;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::graph2pos::{chunk_inplace, g2p};
use std::cmp::min;

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
    eprintln!("daskd {:?} ", g2p(&graph, 1));





    let gu = iterate_test(&graph, 1);
    for x in gu.iter(){
        for y in x.iter(){
            y.printall();
        }
    }
    //println!("{:?}", gu)

}


/// Wrapper for paf files
/// multiple function will be added here
/// E.g. iterate_path is the first function
/// Multithreading base function
/// Output are a list of PAFs
pub fn iterate_test(graph: &NGfa, threads: usize) -> Vec<Vec<Paf>>{
    eprintln!("Iterate test");

    // Get pairs and
    let pairs = get_all_pairs2(graph);
    let chunks = chunk_inplace(pairs, threads);

    let k = Arc::new(g2p(graph, threads));
    println!("jok {:?}", k);
    let k2 = Arc::new(graph.nodes.clone());

    // Resultat
    let mut result = Vec::new();
    let mut rm = Arc::new(Mutex::new(result));
    let mut handles = Vec::new();

    // Iterate over chunks
    for chunk in chunks{
        let r = Arc::clone(&rm);
        let mut r2 = Arc::clone(&k);
        let r3 = Arc::clone(&k2);
        let handle = thread::spawn(move || {
            for pair in chunk.iter(){
                let mut rr = r.lock().unwrap();
                let h = bifurcation_simple(&(&pair.0, &pair.1), &r2, &r3);
                rr.push(h);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap()
    }
    //eprintln!("{:?}", rm.lock().unwrap());
    let i = rm.lock().unwrap().clone();
    i
}


/// Stop the paf when there is more than X "different" sequence
pub fn bifurcation_simple(pair: &(&NPath, &NPath), gfa2pos: &HashMap<String, Vec<usize>>, g2n: &HashMap<u32, NNode>) -> Vec<Paf>{
    let shared = get_shared_direction(pair.0, pair.1);
    let mut paf_vector: Vec<Paf> = Vec::new();
    let shared_vec = get_shared_direction_test(pair.0, pair.1);


    let name1 = pair.0.name.clone();
    let name2 = pair.1.name.clone();
    let maxn1 = gfa2pos.get(&name1).unwrap().last().unwrap().clone();
    let maxn2 = gfa2pos.get(&name2).unwrap().last().unwrap().clone();



    let mut open = false;
    let mut paf_entry = Paf::new(&pair.0.name, &pair.1.name, &0, &0, &0,&0);

    let mut last_index = 0;
    let mut last_i = 0;
    let mut last_shared = 0;
    // Inbetweens are for distance calculation
    let mut distance1: u32 = 0;
    let mut distance2: u32 = 0;





    let mut position = 0;
    println!("{} {}", pair.1.name, pair.0.name);
    // Iterate over each pair
    for x in 0..pair.0.nodes.len() {
        // Check if pair is shared
        let node = &(pair.0.nodes[x], pair.0.dir[x]);
        position += g2n.get(pair.0.nodes.get(x).unwrap()).unwrap().len as u32;

        // Wenn shared
        if shared.contains(node) {
            // Iterate over the other path (for the last shared) and check if it is the same
            distance2 = 0;
            'tt: for y in last_index..pair.1.nodes.len() {

                // If found
                println!("Das is der node {:?}", node);
                println!("Das schau ich gerade an {:?}", (pair.1.nodes[y], pair.1.dir[y]));
                if node == &(pair.1.nodes[y], pair.1.dir[y]) {
                    println!("iii {}", last_i);
                    eprintln!("hit");
                    // If there is a open paf
                    if open {
                        println!("{} {}", distance1, distance2);
                        if (distance1+ distance2) ==0{
                            last_i = x;
                            println!("is zero");
                            paf_entry.flag.flag.push((1,g2n.get(pair.1.nodes.get(y).unwrap()).unwrap().len as u32))
                        } else if (distance1 + distance2) < 20{
                            last_i = x;
                            if distance1 == 0 {
                                paf_entry.flag.flag.push((2,distance2));
                                paf_entry.flag.flag.push((1, g2n.get(pair.1.nodes.get(y).unwrap()).unwrap().len as u32))

                            } else if distance2  == 0 {
                                paf_entry.flag.flag.push((3, distance1));
                                paf_entry.flag.flag.push((1, g2n.get(pair.0.nodes.get(x).unwrap()).unwrap().len as u32))
                            } else if distance2 == distance1{
                                paf_entry.flag.flag.push((4, distance1));
                                paf_entry.flag.flag.push((1, g2n.get(pair.0.nodes.get(x).unwrap()).unwrap().len as u32))
                            } else {
                                let dis = min(distance1, distance2);
                                paf_entry.flag.flag.push((4, dis));
                                if distance2 > distance1{
                                    paf_entry.flag.flag.push((2, distance2-distance1))

                                } else {
                                    paf_entry.flag.flag.push((3,distance1-distance2));
                                }
                                paf_entry.flag.flag.push((1, g2n.get(pair.0.nodes.get(x).unwrap()).unwrap().len as u32))
                            }
                        } else {
                            paf_entry.target_end = gfa2pos.get(&pair.1.name).unwrap()[last_index] as u32;
                            paf_entry.query_end = gfa2pos.get(&pair.0.name).unwrap()[last_i+1] as u32;
                            paf_vector.push(paf_entry.clone());
                            paf_entry = Paf::new(&pair.0.name, &pair.1.name, &(gfa2pos.get(&pair.0.name).unwrap()[x] as u32), &(gfa2pos.get(&pair.1.name).unwrap()[y] as u32), &(maxn1 as u32), &(maxn2 as u32));
                            paf_entry.flag.flag.push((1, g2n.get(pair.0.nodes.get(x).unwrap()).unwrap().len as u32));

                            open = true;
                            last_i = x;
                        }
                    }
                    else {
                        println!("hihi");
                        paf_entry = Paf::new(&pair.0.name, &pair.1.name, &(gfa2pos.get(&pair.0.name).unwrap()[x] as u32), &(gfa2pos.get(&pair.1.name).unwrap()[y] as u32), &(maxn1 as u32), &(maxn2 as u32));

                        paf_entry.flag.flag.push((1, g2n.get(pair.0.nodes.get(x).unwrap()).unwrap().len as u32));
                        open = true;

                    }
                    last_index = y.clone() +1;
                    distance2 = 0;
                    distance1 = 0;
                    break 'tt;
                } else {
                    println!("Nix gefunden bei dem node {} {}", pair.0.nodes[x], pair.0.dir[x]);
                    distance2 += g2n.get(pair.1.nodes.get(y).unwrap()).unwrap().len as u32;
                }
            }
        } else {
            if distance1 > 20{
                if open{
                    paf_entry.target_end = gfa2pos.get(&pair.1.name).unwrap()[last_index] as u32;
                    paf_entry.query_end = gfa2pos.get(&pair.0.name).unwrap()[last_i+1] as u32;
                    paf_vector.push(paf_entry.clone());

                }
                open = false;
            }
            distance1 += g2n.get(pair.0.nodes.get(x).unwrap()).unwrap().len as u32;
        }
    }
    if open{
        paf_entry.target_end = gfa2pos.get(&pair.1.name).unwrap()[last_index] as u32;
        paf_entry.query_end = gfa2pos.get(&pair.0.name).unwrap()[last_i+1] as u32;
        paf_vector.push(paf_entry.clone())
    }
    if paf_vector.len() == 0{
        println!("{} {}", pair.1.name, pair.0.name);
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