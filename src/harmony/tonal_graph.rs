//! Tonal graph.
use serde::{Deserialize,Serialize};
use std::collections::HashMap;

pub const PITCH_NAMES:[&str;12]=["C","C#","D","D#","E","F","F#","G","G#","A","A#","B"];
pub const CIRCLE_OF_FIFTHS:[i32;12]=[0,7,2,9,4,11,6,1,8,3,10,5];

pub fn pitch_name_fn(pc:i32)->Result<&'static str,String>{
    if !(0..=11).contains(&pc){return Err(format!("invalid pc: {}",pc))}
    Ok(PITCH_NAMES[pc as usize])
}
pub fn circle_of_fifths_position(pc:i32)->i32{CIRCLE_OF_FIFTHS.iter().position(|&p|p==pc).unwrap_or(0)as i32}
pub fn semitone_interval(from:i32,to:i32)->i32{let d=(to-from).rem_euclid(12);if d>6{d-12}else{d}}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash,Serialize,Deserialize)]
pub enum TransitionDirection{Dominant,Subdominant,Resolution,Mediant,Step,Tritone,NoMotion,Unknown}

pub fn classify_direction(from:i32,to:i32)->TransitionDirection{
    match(to-from).rem_euclid(12){
        0=>TransitionDirection::NoMotion,7=>TransitionDirection::Dominant,5=>TransitionDirection::Subdominant,
        10=>TransitionDirection::Resolution,2=>TransitionDirection::Step,3|4|8|9=>TransitionDirection::Mediant,
        6=>TransitionDirection::Tritone,_=>TransitionDirection::Step,
    }
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct TonalGraph{edges:HashMap<(i32,i32),f64>,outgoing:HashMap<i32,Vec<i32>>,total_weight:f64}

impl TonalGraph{
    pub fn new()->Self{let mut out=HashMap::new();for pc in 0..12{out.insert(pc,Vec::new());}Self{edges:HashMap::new(),outgoing:out,total_weight:0.0}}
    pub fn add_transition(&mut self,from:i32,to:i32,w:f64){
        let key=(from,to);
        if let Some(e)=self.edges.get_mut(&key){*e+=w;self.total_weight+=w}
        else{self.edges.insert(key,w);self.outgoing.entry(from).or_default().push(to);self.total_weight+=w}
    }
    pub fn build_from_progression(&mut self,roots:&[i32]){for w in roots.windows(2){self.add_transition(w[0],w[1],1.0)}}
    pub fn neighbors(&self,pc:i32)->Vec<i32>{self.outgoing.get(&pc).cloned().unwrap_or_default()}
    pub fn weight(&self,from:i32,to:i32)->f64{self.edges.get(&(from,to)).copied().unwrap_or(0.0)}
    pub fn edge_count(&self)->usize{self.edges.len()}
}

#[cfg(test)]
mod tests{use super::*;#[test]fn t_graph(){let mut g=TonalGraph::new();g.build_from_progression(&[0,5,7,0]);assert_eq!(g.edge_count(),3)}#[test]fn t_name(){assert_eq!(pitch_name_fn(0).unwrap(),"C")}}
