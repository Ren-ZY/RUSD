use crate::johnson_find_cycles::strong_connected_components::*;

/*
pub fn get_adjacency_list(adjacency_matrix: &Vec<Vec<bool>>) -> Vec<Vec<usize>>{
    let mut lr = vec![];
    for i in 0..adjacency_matrix.len() {
        let mut vx = vec![];
        for j in 0..adjacency_matrix[i].len(){
            if adjacency_matrix[i][j] {
                vx.push(j as usize);
            }
        }
        lr.push(vx);
    }
    lr
}
*/

pub struct ElementaryCyclesSearch{
    cycles: Vec<Vec<String>>,
    adj_list: Vec<Vec<usize>>,
    graph_nodes: Vec<String>,
    blocked: Vec<bool>,
    b: Vec<Vec<usize>>,
    stack: Vec<usize>
}

impl ElementaryCyclesSearch{
    pub fn new(matrix: &Vec<Vec<usize>>, g_nodes: &Vec<String>) -> Self{
        Self{
            cycles: vec![],
            adj_list: matrix.clone(),
            graph_nodes: g_nodes.clone(),
            blocked: vec![],
            b: vec![],
            stack: vec![]
        }
    }

    pub fn find_cycles(&mut self, v: usize, s: usize, adj_list: &Vec<Vec<usize>>) -> bool{
        let mut f = false;
        self.stack.push(v);
        self.blocked[v] = true;
        for i in 0..adj_list[v].len() {
            let w = adj_list[v][i];
            // found cycle
            if w == s as usize {
                let mut cycle = Vec::new();
                for j in 0..self.stack.len() {
                    let index = self.stack[j];
                    cycle.push(self.graph_nodes[index].clone());
                }
                self.cycles.push(cycle);
                f = true;
            } else if self.blocked[w] == false{
                if self.find_cycles(w, s, adj_list) == true{
                    f = true;
                }
            }
        }
    
        if f {
            self.unblock(v);
        } else {
            for i in 0..adj_list[v].len() {
                let w = adj_list[v][i];
                if self.b[w].contains(&v) == false{
                    self.b[w].push(v);
                }
            }
        }
    
        self.stack.retain(|x| *x != v);//remove the element v in stack
    
        return f;
    }

    pub fn get_elementary_cycles(&mut self) -> Vec<Vec<String>> {
		
        self.blocked.clear();
        self.b.clear();
        if self.blocked.len() == 0 {
            self.blocked.resize(self.adj_list.len(), false);
        }
        if self.b.len() == 0 {
            self.b.resize(self.adj_list.len(), vec![]);
        }
        let mut sccs = StrongConnectedComponents::new(&self.adj_list);
        let mut s = 0;
        loop {
            let scc_result = sccs.get_adjacency_list(s);
            if scc_result.is_some() && scc_result.as_ref().unwrap().get_adj_list().len() > 0 {
                let scc = scc_result.as_ref().unwrap().get_adj_list();
                    s = scc_result.as_ref().unwrap().get_lowest_node_id();
                    for j in 0..scc.len(){
                        if scc[j].len() > 0 {
                            self.blocked[j] = false;
                            self.b[j].clear();
                    }
                }
                self.find_cycles(s, s, &scc);
                s += 1;
            } else{
                break;
            } 
        }

        self.cycles.clone()
	}

    pub fn unblock(&mut self, node: usize) {
        self.blocked[node] = false;
        //vector<int> bnode = b[node];
        
        while self.b[node].len() > 0 {
            let w = self.b[node][0];
            self.b[node].remove(0);
            if self.blocked[w] {
                self.unblock(w);
            }
        }
    }

}
