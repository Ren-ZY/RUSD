#![allow(dead_code)]

use std::collections::HashSet;

pub struct SCCResult{
    node_ids_of_scc: HashSet<usize>,
    adj_list: Vec<Vec<usize>>,
    lowest_node_id: usize
}

impl SCCResult{
    pub fn new(adj_l: &Vec<Vec<usize>>, l_id: usize) -> Self{
        let mut s = HashSet::new();
        if adj_l.len() > 0{
            for i in l_id..adj_l.len(){
                if adj_l[i as usize].len() > 0 {
                    s.insert(i);
                }
            }
        }
        
        Self{
            adj_list: adj_l.clone(),
            lowest_node_id: l_id,
            node_ids_of_scc: s
        }
    }

    pub fn get_adj_list(&self) -> Vec<Vec<usize>>{
        self.adj_list.clone()
    }

    pub fn get_lowest_node_id(&self) -> usize{
        self.lowest_node_id
    }
}


