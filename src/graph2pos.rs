use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

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



/// Graph to position
/// For each path it get the position for each index (node)
pub fn g2p(graph: & gfaR_wrapper::NGfa, threads: usize) -> HashMap<String, Vec<usize>>{

    let mut result_hm: HashMap<String, Vec<usize>> = HashMap::new();
    let mut result = Arc::new(Mutex::new(result_hm));
    let mut hm = Arc::new(graph.nodes.clone());
    let k = graph.paths.clone();
    let k2 = chunk_inplace(k, threads);
    let mut handles: Vec<_> = Vec::new();
    //println!("sda das {}", k2.len());
    for chunk in k2{
        let mut g2 = Arc::clone(&hm);
        let mut tess1 = Arc::clone(&result);
        let handle = thread::spawn(move || {
            //eprintln!("I spawned");
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
            //eprintln!("Im done");
        });
        handles.push(handle);
        //eprintln!("hello");
    }

    for handle in handles {
        handle.join().unwrap()
    }
    let out =  result.lock().unwrap().clone();
    out

}
