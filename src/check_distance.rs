use gfaR_wrapper::NPath;
use std::collections::{HashSet, HashMap};


pub fn next_index_simple(pairs: &(&NPath, &NPath), index1: &usize, index2: &usize, shared: &HashSet<(u32, bool)>, gfa2pos: &HashMap<String, Vec<usize>>) -> ((usize, usize), (usize, usize)){
    let start1 = index1.clone();
    let start2 = index2.clone();
    let mut dis1:usize = 0;
    let mut dis2:usize = 0;
    let mut dis = 0;
    let mut index = (0,0);
    for x in start1..pairs.0.nodes.len(){
        let node = &(pairs.0.nodes[x], pairs.0.dir[x]);
        if shared.contains(&node){
            for y in start2..pairs.1.nodes.len() {
                if node == &(pairs.1.nodes[y], pairs.1.dir[y]) {
                    index = (x.clone(),y.clone());
                    dis1 = gfa2pos.get(&pairs.0.name).unwrap()[x] - gfa2pos.get(&pairs.0.name).unwrap()[index1.clone()];
                    dis2 = gfa2pos.get(&pairs.1.name).unwrap()[y] - gfa2pos.get(&pairs.1.name).unwrap()[index2.clone()];
                }
            }
        }
    }

    if (dis1 + dis2) > 1000 {
        pafcheck(pairs, &index.0, &index.1, shared);
    }


    (index, (dis1, dis2))
}




/// Return the "next" node
/// --> smallest distance to old shared node
/// Output: (Distance1, Distance2) (index, index)
/// TODO
/// check with hs if iterate or not
pub fn next_index(pairs: &(&NPath, &NPath), index1: &usize, index2: &usize, hs: &HashSet<(u32, bool)>, gfa2pos: &HashMap<String, Vec<usize>>) -> (bool, (usize, usize), (usize, usize)){
    let mut mindis: usize = usize::MAX/3;
    let mut index: (usize, usize) = (0,0);
    let mut dis1:usize = 0;
    let mut dis2:usize = 0;
    let mut dis = 0;
    let mut found = false;
    let mut count = 0;
    for x in index1.clone()..pairs.0.nodes.len(){
        dis = gfa2pos.get(&pairs.0.name).unwrap()[x] - gfa2pos.get(&pairs.0.name).unwrap()[index1.clone()];
        if dis > (mindis*2){
            break;
        }
        let node = &(pairs.0.nodes[x], pairs.0.dir[x]);
        for y in index2.clone()..pairs.1.nodes.len(){
            if node == &(pairs.1.nodes[y], pairs.1.dir[y]){
                dis1 = gfa2pos.get(&pairs.0.name).unwrap()[x] - gfa2pos.get(&pairs.0.name).unwrap()[index1.clone()];
                dis2 = gfa2pos.get(&pairs.1.name).unwrap()[y] - gfa2pos.get(&pairs.1.name).unwrap()[index2.clone()];
                if (dis1 + dis2) < mindis{
                    count += 1;
                    mindis = (dis1+dis2);
                    index = (x.clone(),y.clone());
                    found = true
                }
            }

        }

    }
    if count == 2{
        eprintln!("daskdhasjkhdasjkhdjkash");
    }
    if (dis1 + dis2) > 1000 {
        pafcheck(pairs, &index.0, &index.1, hs);
    }
    (found, (dis1, dis2), index)
}



/// Check if the stretch far away is still good
pub fn pafcheck(pairs: &(&NPath, &NPath), index1: &usize, index2: &usize, hs: &HashSet<(u32, bool)>){
    let mut hs1: HashSet<u32> = HashSet::new();
    let mut hs2: HashSet<u32> = HashSet::new();
    for x in index1.clone()..index1.clone()+100 {
        hs1.insert(pairs.0.nodes[x]);
    }
    for x in index2.clone()..index2.clone()+100 {
        hs2.insert(pairs.1.nodes[x]);
    }

    eprintln!("HS1 {}", hs1.len());
    eprintln!("HS2 {}", hs2.len());
    let o: Vec<u32> = hs1.intersection(&hs2).cloned().collect();
    eprintln!("HSINTER {}", o.len());
}