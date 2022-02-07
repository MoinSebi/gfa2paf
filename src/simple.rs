use gfaR_wrapper::{NPath, NNode};
use std::collections::HashMap;
use crate::core::get_shared_direction;
use crate::paf::Paf;
use std::cmp::min;
use crate::check_distance::next_index;

/// Stop the paf when there is more than X "different" sequence
pub fn bifurcation_simple(pair: &(&NPath, &NPath), gfa2pos: &HashMap<String, Vec<usize>>, g2n: &HashMap<u32, NNode>, maxdistance: usize) -> Vec<Paf>{
    let shared = get_shared_direction(pair.0, pair.1);
    let mut paf_vector: Vec<Paf> = Vec::new();


    let name1 = pair.0.name.clone();
    let name2 = pair.1.name.clone();
    let maxn1 = gfa2pos.get(&name1).unwrap().last().unwrap().clone();
    let maxn2 = gfa2pos.get(&name2).unwrap().last().unwrap().clone();



    let mut open = false;
    let mut paf_entry = Paf::new(&pair.0.name, &pair.1.name, &0, &0, &0,&0);

    let mut last_index = 0;
    let mut last_i = 0;
    // Inbetweens are for distance calculation
    let mut distance1: u32 = 0;
    let mut distance2: u32;




    // Iterate over each pair
    for x in 0..pair.0.nodes.len() {
        // Check if pair is shared
        eprintln!("{}", x);
        let node = &(pair.0.nodes[x], pair.0.dir[x]);

        let ii = next_index(pair, &last_i, &last_index, &shared, &gfa2pos);
        // //eprintln!("{:?} {:?}", ii, (last_i, last_index));
        //
        // if ii != (last_i, last_index){
        //     eprintln!("dasjkdhjsakdas");
        // }

        // Wenn shared
        if shared.contains(node) {
            // Iterate over the other path (for the last shared) and check if it is the same
            distance2 = 0;
            'tt: for y in last_index..pair.1.nodes.len() {

                // If found
                if node == &(pair.1.nodes[y], pair.1.dir[y]) {

                    // If there is a open paf
                    if open {
                        if (distance1+ distance2) ==0{
                            paf_entry.flag.push((1,g2n.get(pair.1.nodes.get(y).unwrap()).unwrap().len as u32))
                        } else if (distance1 + distance2) < maxdistance as u32{
                            if distance1 == 0 {
                                paf_entry.flag.push((2,distance2));
                                paf_entry.flag.push((1, g2n.get(pair.1.nodes.get(y).unwrap()).unwrap().len as u32))

                            } else if distance2  == 0 {
                                paf_entry.flag.push((3, distance1));
                                paf_entry.flag.push((1, g2n.get(pair.0.nodes.get(x).unwrap()).unwrap().len as u32))
                            } else if distance2 == distance1{
                                paf_entry.flag.push((4, distance1));
                                paf_entry.flag.push((1, g2n.get(pair.0.nodes.get(x).unwrap()).unwrap().len as u32))
                            } else {
                                let dis = min(distance1, distance2);
                                paf_entry.flag.push((4, dis));
                                if distance2 > distance1{
                                    paf_entry.flag.push((2, distance2-distance1))

                                } else {
                                    paf_entry.flag.push((3,distance1-distance2));
                                }
                                paf_entry.flag.push((1, g2n.get(pair.0.nodes.get(x).unwrap()).unwrap().len as u32))
                            }

                            // Closing paf and create new one
                        } else {
                            paf_entry.target_end = gfa2pos.get(&pair.1.name).unwrap()[last_index] as u32;
                            paf_entry.query_end = gfa2pos.get(&pair.0.name).unwrap()[last_i+1] as u32;
                            paf_vector.push(paf_entry.clone());

                            // New paf
                            paf_entry = Paf::new(&pair.0.name, &pair.1.name, &(gfa2pos.get(&pair.0.name).unwrap()[x] as u32), &(gfa2pos.get(&pair.1.name).unwrap()[y] as u32), &(maxn1 as u32), &(maxn2 as u32));
                            paf_entry.flag.push((1, g2n.get(pair.0.nodes.get(x).unwrap()).unwrap().len as u32));

                            // Still open and update last_i
                            open = true;
                        }
                    }
                    // there is nothing open
                    // Open a new
                    else {
                        paf_entry = Paf::new(&pair.0.name, &pair.1.name, &(gfa2pos.get(&pair.0.name).unwrap()[x] as u32), &(gfa2pos.get(&pair.1.name).unwrap()[y] as u32), &(maxn1 as u32), &(maxn2 as u32));

                        paf_entry.flag.push((1, g2n.get(pair.0.nodes.get(x).unwrap()).unwrap().len as u32));
                        open = true;

                    }

                    // No matter what: new thing i snew
                    // and the distance is reset
                    // Dont go on with searching
                    last_index = y.clone() +1;
                    last_i = x;
                    distance2 = 0;
                    distance1 = 0;
                    break 'tt;


                    // Not found in second genome
                } else {
                    eprintln!("nothing");
                    if open{
                        distance2 += g2n.get(pair.1.nodes.get(y).unwrap()).unwrap().len as u32;
                        if (distance1 + distance2) > maxdistance as u32{
                            if open{
                                paf_entry.target_end = gfa2pos.get(&pair.1.name).unwrap()[last_index] as u32;
                                paf_entry.query_end = gfa2pos.get(&pair.0.name).unwrap()[last_i+1] as u32;
                                paf_vector.push(paf_entry.clone());
                                distance1 = 0;
                                distance2 = 0;

                            }
                            open = false;

                            break 'tt;
                        }
                    }
                }
            }



            // The first genome has new genome
        } else {
            if open{

                if distance1 > maxdistance as u32{
                    paf_entry.target_end = gfa2pos.get(&pair.1.name).unwrap()[last_index] as u32;
                    paf_entry.query_end = gfa2pos.get(&pair.0.name).unwrap()[last_i+1] as u32;
                    paf_vector.push(paf_entry.clone());
                    open = false;

                } else {
                    distance1 += g2n.get(pair.0.nodes.get(x).unwrap()).unwrap().len as u32;
                }
            }
        }
    }

    // Add "open" pafs to the data set
    if open{
        paf_entry.target_end = gfa2pos.get(&pair.1.name).unwrap()[last_index] as u32;
        paf_entry.query_end = gfa2pos.get(&pair.0.name).unwrap()[last_i+1] as u32;
        paf_vector.push(paf_entry.clone())
    }
    paf_vector
}