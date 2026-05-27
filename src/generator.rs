//! Counterpoint generation via backtracking.

use crate::exceptions::CounterpointError;
use crate::interval::OCTAVE;
use crate::rules;
use serde::{Deserialize,Serialize};

#[derive(Debug,Clone,Copy,PartialEq,Eq,Serialize,Deserialize)]
pub enum Species{First,Second,Third,Fourth,Fifth}

#[derive(Debug,Clone,Copy,Serialize,Deserialize)]
pub struct VoiceRange{pub low:i32,pub high:i32}
impl VoiceRange{pub fn new(l:i32,h:i32)->Self{Self{low:l,high:h}}pub fn contains(&self,n:i32)->bool{n>=self.low&&n<=self.high}}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Scale{pub tonic:i32,pub intervals:Vec<i32>}
impl Scale{
    pub fn major(t:i32)->Self{Self{tonic:t,intervals:vec![0,2,4,5,7,9,11]}}
    pub fn minor(t:i32)->Self{Self{tonic:t,intervals:vec![0,2,3,5,7,8,10]}}
    pub fn pitch_classes(&self)->Vec<i32>{self.intervals.iter().map(|&i|(self.tonic+i)%OCTAVE).collect()}
    pub fn notes_in_range(&self,r:&VoiceRange)->Vec<i32>{
        let pcs=self.pitch_classes();let mut ns=Vec::new();
        for o in -1i32..10{for &pc in &pcs{let n=o*OCTAVE+pc;if r.contains(n){ns.push(n)}}}
        ns.sort();ns
    }
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct CounterpointResult{pub cantus_firmus:Vec<i32>,pub counterpoint:Vec<i32>,pub species:Species,pub success:bool,pub backtrack_count:usize}

pub struct CounterpointGenerator{pub species:Species,pub voice_range:VoiceRange,pub scale:Scale,pub max_backtracks:usize}
impl CounterpointGenerator{
    pub fn new(sp:Species,vr:VoiceRange,sc:Scale)->Self{Self{species:sp,voice_range:vr,scale:sc,max_backtracks:10000}}
    pub fn generate(&self,cf:&[i32])->Result<CounterpointResult,CounterpointError>{
        if cf.is_empty(){return Err(CounterpointError::InvalidInput("empty".into()))}
        let cands=self.scale.notes_in_range(&self.voice_range);
        let mut cp=Vec::with_capacity(cf.len());let mut bt=0;
        let ok=self.fill(cf,&cands,&mut cp,&mut bt);
        Ok(CounterpointResult{cantus_firmus:cf.to_vec(),counterpoint:cp,species:self.species,success:ok,backtrack_count:bt})
    }
    fn fill(&self,cf:&[i32],cands:&[i32],cp:&mut Vec<i32>,bt:&mut usize)->bool{
        if cp.len()==cf.len(){return true}if *bt>self.max_backtracks{return false}
        let idx=cp.len();let cn=cf[idx];
        for &c in cands{if !self.valid(cf,cp,cn,c){continue}cp.push(c);if self.fill(cf,cands,cp,bt){return true}cp.pop();*bt+=1;if *bt>self.max_backtracks{return false}}
        false
    }
    fn valid(&self,cf:&[i32],cp:&[i32],cn:i32,cpn:i32)->bool{
        if !self.voice_range.contains(cpn){return false}
        if !rules::consonant_check(cn,cpn){return false}
        if !cp.is_empty(){
            let pp=cp[cp.len()-1];let pc=cf[cp.len()-1];
            if (cpn-pp).abs()>12{return false}
            let pi=(pc-pp).abs()%OCTAVE;let ci=(cn-cpn).abs()%OCTAVE;
            if pi==7&&ci==7{let da=(cn-pc).signum();let db=(cpn-pp).signum();if da!=0&&da==db{return false}}
            if pi==0&&ci==0{let da=(cn-pc).signum();let db=(cpn-pp).signum();if da!=0&&da==db{return false}}
            if cpn>=cn{return false}
        }
        true
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]fn t_scale(){let s=Scale::major(0);assert!(s.pitch_classes().contains(&0));assert!(!s.pitch_classes().contains(&1));}
    #[test]fn t_gen(){let g=CounterpointGenerator::new(Species::First,VoiceRange::new(48,72),Scale::major(0));let r=g.generate(&[60,62,64,65,67]).unwrap();assert!(r.success);assert_eq!(r.counterpoint.len(),5);}
    #[test]fn t_empty(){let g=CounterpointGenerator::new(Species::First,VoiceRange::new(48,72),Scale::major(0));assert!(g.generate(&[]).is_err());}
}
