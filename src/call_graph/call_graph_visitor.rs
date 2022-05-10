use rustc_hir::def_id::DefId;
use rustc_middle::mir;
use rustc_middle::ty::{TyCtxt, TyKind, Instance, InstanceDef};
use crate::call_graph::call_graph_builder::*;
use crate::progress_info;
use std::fmt::Write;

pub fn is_std_crate(crate_name: &String) -> bool {
     crate_name.as_str() == "alloc" ||
         crate_name.as_str() == "std" ||
         crate_name.as_str() == "core" ||
         crate_name.as_str() == "proc_macro" ||
         crate_name.as_str() == "clippy"
}


pub fn get_fn_path(tcx: &TyCtxt, def_id: DefId) -> String{
    let mut out = String::new();
    let res = write!(&mut out, "{:?}", tcx.def_path_debug_str(def_id));
    match res {
        Ok(()) => {out}
        Err(_e) => {panic!("Get DefPath Error!");}
    }
}

pub fn get_fn_location(tcx: &TyCtxt, def_id: DefId) -> String{
    let mut out = String::new();
    let res = write!(&mut out, "{:?}", tcx.def_span(def_id));
    match res {
        Ok(()) => {out}
        Err(_e) => {panic!("Get DefPath Error!");}
    }
}

pub struct CallGraphVisitor<'b, 'tcx>{
   tcx: TyCtxt<'tcx>,
   def_id: DefId,
   body: &'tcx  mir::Body<'tcx>,
   call_graph_info: &'b mut CallGraphInfo,
}

impl<'b, 'tcx> CallGraphVisitor<'b, 'tcx>{
    pub fn new(tcx: TyCtxt<'tcx>, def_id: DefId, body: &'tcx mir::Body<'tcx>, 
            call_graph_info: &'b mut CallGraphInfo) -> Self{
        Self{
           tcx: tcx,
           def_id: def_id,
           body: body,
           call_graph_info: call_graph_info,
        }
    }
    pub fn add_in_call_graph(&mut self, caller_def_path: &String, callee_def_id: DefId, callee_def_path: &String){
        if let Some(caller_id) = self.call_graph_info.get_node_by_def_path(caller_def_path){
            if let Some(callee_id) = self.call_graph_info.get_node_by_def_path(callee_def_path){    
                self.call_graph_info.add_function_call_edge(caller_id, callee_id);           
            }
            else{
                self.call_graph_info.add_node(callee_def_id, callee_def_path);
                if let Some(callee_id) = self.call_graph_info.get_node_by_def_path(callee_def_path){
                    self.call_graph_info.add_function_call_edge(caller_id, callee_id);
            }
         }
      } 
    }
    pub fn visit(&mut self){
       let caller_def_path = get_fn_path(&self.tcx, self.def_id);
       self.call_graph_info.add_node(self.def_id, &caller_def_path);
       for(_basic_block_index, basic_block_data) in self.body.basic_blocks().iter_enumerated(){
           let terminator = basic_block_data.terminator();
           self.visit_terminator(&terminator);
       }
    }
      
    fn visit_terminator(
        &mut self,
        terminator: & mir::Terminator<'tcx>,
    ){
        match &terminator.kind{
            mir::TerminatorKind::Call {
                func,
                ..  
            } => {
                match func{
                    mir::Operand::Constant(constant) => {
                        if let TyKind::FnDef(callee_def_id, callee_substs) = constant.literal.ty.kind{
                             if !is_std_crate(&self.tcx.crate_name(callee_def_id.krate).to_string()){ 
                                 let param_env = self.tcx.param_env(self.def_id);
                                 if let Ok(Some(instance)) = Instance::resolve(self.tcx, param_env, callee_def_id, callee_substs){
                                     let mut instance_def_id = None;
                                     match instance.def{
                                         InstanceDef::Item(def_id) => {
                                             instance_def_id = Some(def_id.def_id_for_type_of());
                                        // println!("instance_callee_def_path: {}", get_fn_path(&self.tcx, instance_def_id.def_id_for_type_of()));
                                         }
                                         InstanceDef::Intrinsic(def_id)
                                         | InstanceDef::CloneShim(def_id, _) => {
                                             if !self.tcx.is_closure(def_id){
                                                 instance_def_id = Some(def_id);
                                             // println!("instance_callee_def_path: {}", get_fn_path(&self.tcx, instance_def_id));
                                             } 
                                         }
                                         _ => {}
                                     }
                                     if let Some(instance_def_id) = instance_def_id{
                                         if instance_def_id == self.def_id{
                                             let caller_def_path = get_fn_path(&self.tcx, self.def_id);
                                             let callee_def_path = get_fn_path(&self.tcx, instance_def_id); 
                                             let location = get_fn_location(&self.tcx, instance_def_id);
                                             let msg = "\x1b[031mwarning!! find a recursion function which may cause stackoverflow\x1b[0m";
                                             println!("{}", instance);
                                             progress_info!("{}: {}->{}; \x1b[031mlocation\x1b[0m: {}", msg, caller_def_path, callee_def_path, location); 
                                         }
                                         let caller_def_path = get_fn_path(&self.tcx, self.def_id);
                                         let callee_def_path = get_fn_path(&self.tcx, instance_def_id);
                                        // let location = get_fn_location(&self.tcx, instance_def_id);
                                        // println!("instance_callee_def_path: {}; location: {}", callee_def_path, location);
                                         self.add_in_call_graph(&caller_def_path, instance_def_id, &callee_def_path);
                                     }
                                 }
                                 else{
                                     if self.def_id == callee_def_id{
                                         let caller_def_path = get_fn_path(&self.tcx, self.def_id);
                                         let callee_def_path = get_fn_path(&self.tcx, callee_def_id); 
                                         let location = get_fn_location(&self.tcx, callee_def_id);  
                                         let msg = "\x1b[031mwarning!! find a recursion function which may cause stackoverflow\x1b[0m";
                                         progress_info!("{}: {}->{}; \x1b[031mlocation\x1b[0m: {}", msg, caller_def_path, callee_def_path,location); 
                                     }
                                     let caller_def_path = get_fn_path(&self.tcx, self.def_id);
                                     let callee_def_path = get_fn_path(&self.tcx, callee_def_id);
                                     //let location = get_fn_location(&self.tcx, callee_def_id);
                                     //println!("callee: {}; location: {}", callee_def_path, location);
                                     self.add_in_call_graph(&caller_def_path, callee_def_id, &callee_def_path);
                                 }
                             }
                        }
                    }
                    _ => {}
                 } 
              }
            _ => {}
        }
    }
}
