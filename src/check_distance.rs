use gfaR_wrapper::NPath;
use std::collections::{HashSet, HashMap};

pub fn next_index(pairs: &(&NPath, &NPath), index1: &usize, index2: &usize, hs: &HashSet<(u32, bool)>, gfa2pos: &HashMap<String, Vec<usize>>) -> (usize, usize){
    let mut mindis: usize = usize::MAX/3;
    let mut index: (usize, usize) = (0,0);
    let mut dis1:usize = 0;
    let mut dis2:usize = 0;
    let mut dis = 0;
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
                    mindis = (dis1+dis2);
                    index = (x.clone(),y.clone());
                }
            }

        }

    }
    index
}