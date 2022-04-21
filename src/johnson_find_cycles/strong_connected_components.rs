use crate::johnson_find_cycles::scc_result::*;

pub struct StrongConnectedComponents{
    adj_list_original: Vec<Vec<usize>>,
    adj_list: Vec<Vec<usize>>,
    visited: Vec<bool>,
    stack: Vec<usize>,
    low_link: Vec<usize>,
    number: Vec<usize>,
    scc_counter: usize,
    current_sccs: Vec<Vec<usize>>
}

impl StrongConnectedComponents{
    pub fn new(adj_list: &Vec<Vec<usize>>) -> Self{
        Self{
            adj_list_original: adj_list.clone(),
            adj_list: vec![],
            visited: vec![],
            stack: vec![],
            low_link: vec![],
            number: vec![],
            scc_counter: 0,
            current_sccs: vec![]
        }
    }

    pub fn get_adjacency_list(&mut self, node: usize) -> Option<SCCResult>{
        self.visited.clear();
        self.low_link.clear();
        self.number.clear();
        self.stack.clear();
        self.current_sccs.clear();
        if self.visited.len() == 0 {
            self.visited.resize(self.adj_list_original.len(), false);
        }
        if self.low_link.len() == 0{
            self.low_link.resize(self.adj_list_original.len(), 0);
        }
        if self.number.len() == 0{
            self.number.resize(self.adj_list_original.len(), 0);
        }
        
        self.make_adj_list_subgraph(node);

        for i in node..self.adj_list_original.len(){
            if !self.visited[i] {
                self.get_sccs(i);
                let nodes = self.get_lowest_id_component();
                if nodes.is_some() && !nodes.as_ref().unwrap().contains(&node) && !nodes.as_ref().unwrap().contains(&(node+1)) {
                    return self.get_adjacency_list(node+1);
                }else{
                    let adjacency_list = self.get_adj_list(&nodes);
                    if let Some(adjacency_list) = adjacency_list{
                        for j in 0.. self.adj_list_original.len(){
                            if adjacency_list[j].len() > 0{
                                return Some(SCCResult::new(&adjacency_list, j));
                            }
                        }
                    }
                }
            }
        }
        return None;
    }

    pub fn make_adj_list_subgraph(&mut self, node: usize){
        self.adj_list.clear();
        if self.adj_list.len() == 0{
            self.adj_list.resize(self.adj_list_original.len(), vec![]);
        }
        for i in node..self.adj_list.len(){
            let mut successors = vec![];
            for j in 0..self.adj_list_original[i].len(){
                if self.adj_list_original[i][j] >= node {
                    successors.push(self.adj_list_original[i][j]);
                }
            }
            if successors.len() > 0 {
                self.adj_list[i].resize(successors.len(), 0);
                for j in 0..successors.len(){
                    let succ = successors[j];
                    self.adj_list[i][j] = succ;
                }
            }
        }
    }

    pub fn get_lowest_id_component(&mut self) -> Option<Vec<usize>>{
        let mut min = self.adj_list.len();
        let mut curr_scc = None;
        for i in 0..self.current_sccs.len() {
            let scc = &self.current_sccs[i];
            for j in 0..scc.len() {
                let node = scc[j];
                if node < min {
                    curr_scc = Some(scc.clone());
                    min = node;
                }
            }
        }
        return curr_scc;
    }

    pub fn get_adj_list(&mut self, nodes: &Option<Vec<usize>>) -> Option<Vec<Vec<usize>>> {
        let mut lowest_id_adjacency_list = None;
        if let Some(nodes) = nodes {
           lowest_id_adjacency_list = Some(vec![vec![]; self.adj_list.len()]);
           for i in 0..nodes.len(){
               let node = nodes[i];
               for j in 0..self.adj_list[node].len() {
                   let succ = self.adj_list[node][j];
                   if nodes.contains(&succ){
                       if let Some(ref mut lowest_id_adjacency_list) = lowest_id_adjacency_list{
                           lowest_id_adjacency_list[node].push(succ);
                       }
                   } 
               }
           }    
        }
    return lowest_id_adjacency_list;
    }

    pub fn get_sccs(&mut self, root: usize) {
        self.scc_counter += 1;
        self.low_link[root] = self.scc_counter;
        self.number[root] = self.scc_counter;
        self.visited[root] = true;
        self.stack.push(root);
        for i in 0..self.adj_list[root].len(){
            let w = self.adj_list[root][i];
            if !self.visited[w]{
                self.get_sccs(w);
                self.low_link[root] = if self.low_link[root] < self.low_link[w]{
                    self.low_link[root]
                }else{
                    self.low_link[w]
                }

            } else if self.number[w] < self.number[root] {
                if self.stack.contains(&w) {
                    self.low_link[root] = if self.low_link[root] < self.number[w]{
                        self.low_link[root]
                    }else{
                        self.number[w]
                    }
                }
            }
        }

        //found scc
        if self.low_link[root] == self.number[root] && self.stack.len() > 0 {
            let mut scc = vec![];
            let mut next = self.stack[self.stack.len()-1];
            self.stack.remove(self.stack.len()-1);
            scc.push(next);
    
            while self.number[next] > self.number[root] {
                next = self.stack[self.stack.len()-1];
                self.stack.remove(self.stack.len()-1);
                scc.push(next);
            }
            
            if scc.len() > 1{
                self.current_sccs.push(scc);
            }
        }
    }
}
