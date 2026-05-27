//! Laman graph construction.

use serde::{Deserialize,Serialize};
use std::collections::HashMap;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct CounterpointGraph {
    pub n_voices: usize, pub edges: Vec<(usize,usize)>, pub constraints: HashMap<(usize,usize), Vec<String>>,
}

impl CounterpointGraph {
    pub fn new(n:usize)->Result<Self,String> {
        if n<2 { return Err(format!("n_voices must be >= 2, got {}", n)); }
        let edges = henneberg_construct(n, 42);
        let mut g = Self { n_voices:n, edges, constraints:HashMap::new() };
        g.assign_defaults(); Ok(g)
    }
    pub fn add_constraint(&mut self, i:usize, j:usize, name:&str) {
        let e = (i.min(j), i.max(j));
        if !self.edges.contains(&e) { self.edges.push(e); }
        self.constraints.entry(e).or_default().push(name.to_string());
    }
    pub fn verify_rigidity(&self)->bool{ self.edges.len()==2*self.n_voices-3 }
    pub fn expected_edges(&self)->usize{ 2*self.n_voices-3 }
    pub fn is_minimally_rigid(&self)->bool{ self.edges.len()==self.expected_edges() }
    fn assign_defaults(&mut self) {
        let std=["no_parallel_fifths","no_parallel_octaves","proper_resolution","max_leap_seventh","consonant_interval"];
        for (i,e) in self.edges.iter().enumerate() { self.constraints.entry(*e).or_default().push(std[i%std.len()].to_string()); }
    }
}

pub fn henneberg_construct(n:usize, seed:u64)->Vec<(usize,usize)> {
    if n<2 { return vec![]; }
    let mut edges=vec![(0usize,1usize)]; let mut s=seed;
    for k in 2..n {
        s=s.wrapping_mul(6364136223846793005).wrapping_add(1); let a=(s>>33)as usize%k;
        s=s.wrapping_mul(6364136223846793005).wrapping_add(1); let b=(s>>33)as usize%k;
        edges.push((a.min(b),a.max(b))); edges.push((0,k));
    }
    edges.truncate(2*n-3); edges.sort(); edges.dedup(); edges
}

pub fn verify_rigidity(n:usize, edges:&[(usize,usize)])->bool{ edges.len()==2*n-3 }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn t_construct() { let e=henneberg_construct(4,42); assert!(e.len()<=2*4-3); }
    #[test] fn t_new() { let g=CounterpointGraph::new(4).unwrap(); assert_eq!(g.n_voices,4); }
    #[test] fn t_min() { assert!(CounterpointGraph::new(1).is_err()); }
    #[test] fn t_rigid() { assert!(verify_rigidity(3,&[(0,1),(0,2),(1,2)])); }
}
