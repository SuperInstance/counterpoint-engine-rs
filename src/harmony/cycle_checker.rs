//! Cycle checker.
use super::tonal_graph::{circle_of_fifths_position,classify_direction,semitone_interval,TransitionDirection};
use serde::{Deserialize,Serialize};

#[derive(Debug,Clone,Copy,PartialEq,Eq,Serialize,Deserialize)]
pub enum ProgressionType{Diatonic,ModalInterchange,Modulation,ChromaticMediant,Chromatic,Unknown}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct HolonomyResult{pub holonomy:i32,pub winding_number:f64,pub max_deviation:i32,pub progression_type:ProgressionType,pub steps:Vec<(i32,i32,TransitionDirection)>,pub cumulative:Vec<i32>}
impl HolonomyResult{pub fn is_consistent(&self)->bool{self.holonomy==0}}

fn co5_step(from:i32,to:i32)->i32{let p1=circle_of_fifths_position(from);let p2=circle_of_fifths_position(to);let mut d=p2-p1;if d>6{d-=12}else if d< -6{d+=12};d}

pub fn compute_holonomy(roots:&[i32],wrap:bool)->HolonomyResult{
    assert!(!roots.is_empty());let mut path=roots.to_vec();if wrap&&path.len()>1{path.push(path[0])}
    let mut steps=Vec::new();let mut cum=vec![0i32];let mut total=0i32;let mut max_dev=0i32;
    for w in path.windows(2){let dir=classify_direction(w[0],w[1]);let s=co5_step(w[0],w[1]);total+=s;cum.push(total);max_dev=max_dev.max(total.abs());steps.push((w[0],w[1],dir))}
    let winding=total as f64/12.0;let mut hol=(total*7).rem_euclid(12);if hol>6{hol-=12}
    let pt=classify_sig(hol,max_dev,winding,&steps);
    HolonomyResult{holonomy:hol,winding_number:winding,max_deviation:max_dev,progression_type:pt,steps,cumulative:cum}
}

fn classify_sig(hol:i32,max_dev:i32,wind:f64,steps:&[(i32,i32,TransitionDirection)])->ProgressionType{
    if hol==0&&max_dev<=2{return ProgressionType::Diatonic}
    let tc=steps.iter().filter(|&&(a,b,d)|d==TransitionDirection::Mediant&&[3,4].contains(&semitone_interval(a,b).abs())).count();
    if tc>=2&&max_dev>=3{return ProgressionType::ChromaticMediant}
    if hol==0&&max_dev>2{return ProgressionType::ModalInterchange}
    if wind.abs()>=0.5||hol.abs()>=3{return ProgressionType::Modulation}
    if max_dev>=4{return ProgressionType::Chromatic}
    ProgressionType::Unknown
}

pub fn winding_number(roots:&[i32])->f64{if roots.len()<2{0.0}else{compute_holonomy(roots,false).winding_number}}
pub fn classify_progression(roots:&[i32])->ProgressionType{compute_holonomy(roots,false).progression_type}

#[cfg(test)]mod tests{use super::*;#[test]fn t_holonomy(){let r=compute_holonomy(&[0,5,7,0],true);assert!(!r.steps.is_empty())}#[test]fn t_winding(){let w=winding_number(&[0,7,2,9,4,11,0]);assert!(w>=0.0)}}
