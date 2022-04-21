use rustc_hir::def_id::{DefId, LOCAL_CRATE};
use rustc_middle::ty::TyCtxt;
use crate::call_graph::call_graph_visitor::*;
use std::collections::HashMap;
use std::cell::RefCell;
use crate::johnson_find_cycles::elementary_cycles_search::*;
use crate::progress_info; 

#[derive(Clone)]
pub struct Node{
    def_id: DefId,
    def_path: String,
}

impl Node{
    pub fn new(def_id: DefId, def_path: &String) -> Self{
        Self{
            def_id: def_id,
            def_path: def_path.clone(),
        }
    }
    pub fn get_def_id(&self) -> DefId{
        self.def_id
    }
    pub fn get_def_path(&self) -> String{
        self.def_path.clone()
    }
    }

#[derive(Clone)]
pub struct CallGraphInfo{
    pub functions: RefCell<HashMap<usize, Node>>,
    pub function_calls: RefCell<Vec<(usize, usize)>>,
    pub node_registry: RefCell<HashMap<String, usize>>
}

impl CallGraphInfo{
    pub fn new() -> Self{
        Self {
            functions: RefCell::new(HashMap::new()),
            function_calls: RefCell::new(Vec::new()),
            node_registry: RefCell::new(HashMap::new()),
        }
    }
    pub fn get_node_num(&self) -> usize{
        self.functions.borrow().len()
    }

    pub fn add_node(&self, def_id: DefId, def_path: &String){
        if let None = self.get_node_by_def_path(def_path){
            let id = self.node_registry.borrow().len();
            //println!("{}", id);
            let node = Node::new(def_id, def_path);
            self.node_registry.borrow_mut().insert(def_path.clone(), id);
            self.functions.borrow_mut().insert(id, node);
        }
    }
    pub fn add_function_call_edge(&self, caller_id: usize, callee_id: usize){
        self.function_calls.borrow_mut().push((caller_id, callee_id));
    }
    pub fn get_node_by_def_path(&self, def_path: &String) -> Option<usize>{
        if let Some(&id) = self.node_registry.borrow().get(def_path){
               Some(id)
        }
        else{
            None
        }
    }
    
    pub fn print_call_graph(&self){
        println!("There are {} function calls!!", self.function_calls.borrow().len());
        for function_call in self.function_calls.borrow().clone(){
            let caller_id = function_call.0;
            let callee_id = function_call.1;
            if let Some(caller_node) = self.functions.borrow().get(&caller_id) {
               if let Some(callee_node) = self.functions.borrow().get(&callee_id){
                   let caller_def_path = caller_node.get_def_path();
                   let callee_def_path = callee_node.get_def_path();
                   println!("{}:{}->{}:{}", function_call.0, caller_def_path, function_call.1, callee_def_path);     
               } 
            }
        }
        println!("There are {} functions", self.functions.borrow().len());
        for (key, value) in self.functions.borrow().clone(){
            println!("{}:{}", key, value.get_def_path());
        }
    }
}

pub fn call_graph_builder(tcx: TyCtxt) -> CallGraphInfo{
    let mut call_graph_info = CallGraphInfo::new();
    
    for &def_id in tcx.mir_keys(LOCAL_CRATE).iter(){
       // let caller_def_path = get_fn_path(&tcx, def_id.to_def_id());
       // println!("caller_def_path: {}", caller_def_path);
        let body = &tcx.optimized_mir(def_id);
        let mut call_graph_visitor = CallGraphVisitor::new(tcx.clone(), def_id.to_def_id(), body, &mut call_graph_info);
        call_graph_visitor.visit();
        
    }
    call_graph_info
}

pub fn get_adj_list_and_find_cycles(tcx: TyCtxt, call_graph_info: &CallGraphInfo){
    let num = call_graph_info.get_node_num();
    let mut call_graph_adj_list = vec![vec![]; num];
    let mut nodes = vec![String::new(); num];
    for (caller, callee) in call_graph_info.function_calls.borrow().iter(){
        call_graph_adj_list[*caller].push(*callee); 
    }
    for (id, node) in call_graph_info.functions.borrow().iter(){
        nodes[*id] = node.get_def_path();
    }
    let mut ecs = ElementaryCyclesSearch::new(&call_graph_adj_list, &nodes);
    let cycles = ecs.get_elementary_cycles();
    for i in 0..cycles.len(){
        if cycles[i].len() > 1{
        let mut res = String::new();
        let cycle = cycles[i].clone();
        for j in 0..cycle.len(){
            let node = cycle[j].clone();
            if j < cycle.len() - 1 {
                //print!("{}->", node);
                res.push_str(&node);
                res.push_str("->");
            }else{
                res.push_str(&node);
                res.push_str("->");
                res.push_str(&cycle[0].clone());
            }
        }
        let msg = "\x1b[031mwarning!! find a recursion function which may cause stackoverflow\x1b[0m";
        if let Some(id) = call_graph_info.get_node_by_def_path(&cycle[0]){
            if let Some(first_node) = call_graph_info.functions.borrow().get(&id){
                let first_node_def_id = first_node.get_def_id();
                let location = get_fn_location(&tcx, first_node_def_id);
                progress_info!("{}:{}; \x1b[031mlocation:\x1b[0m {}", msg, res, location);   
            }
        }
      }
    }
}
