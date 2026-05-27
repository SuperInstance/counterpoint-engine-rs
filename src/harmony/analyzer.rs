//! Chord progression analysis.
use super::cycle_checker::compute_holonomy;
use super::tonal_graph::{pitch_name_fn,TonalGraph};
use serde::{Deserialize,Serialize};

const MAJOR_SCALE:[i32;7]=[0,2,4,5,7,9,11];
const MINOR_SCALE:[i32;7]=[0,2,3,5,7,8,10];
const MAJOR_QUALITIES:[&str;7]=["maj","min","min","maj","maj","min","dim"];

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Chord{pub root:i32,pub quality:String,pub function:String,pub key:(i32,String),pub is_diatonic:bool,pub is_secondary_dominant:bool,pub implied_key:(i32,String)}
impl Chord{pub fn root_name(&self)->String{pitch_name_fn(self.root).unwrap_or("?").to_string()}}

#[derive(Debug,Clone)]
pub struct ProgressionAnalysis{pub chords:Vec<Chord>,pub graph:TonalGraph,pub modulations:Vec<(usize,String)>,pub modal_interchanges:Vec<(usize,String)>,pub stability_score:f64}

pub fn parse_roman(symbol:&str,key_tonic:i32,mode:&str)->Chord{
    let ml=mode.to_lowercase();let scale: &[i32;7]=if ml=="major"{&MAJOR_SCALE}else{&MINOR_SCALE};
    if symbol.contains('/')&&!symbol.starts_with('b')&&!symbol.starts_with('#'){
        if let Some(pos)=symbol.find('/'){
            let tc=parse_roman(&symbol[pos+1..],key_tonic,&ml);let dc=parse_roman(&symbol[..pos],tc.root,"major");
            return Chord{root:dc.root,quality:dc.quality,function:symbol.to_string(),key:(key_tonic,ml),is_diatonic:false,is_secondary_dominant:true,implied_key:(tc.root,"major".to_string())};
        }
    }
    let(acc,num)=parse_acc(symbol);let degree=num_to_degree(num);let mut root=(key_tonic+scale[degree])%12;root=apply_acc(root,&acc);
    let quality=MAJOR_QUALITIES.get(degree).unwrap_or(&"maj").to_string();let is_diatonic=scale.iter().any(|&s|(key_tonic+s)%12==root)&&acc.is_empty();
    Chord{root,quality,function:symbol.to_string(),key:(key_tonic,ml.clone()),is_diatonic,is_secondary_dominant:false,implied_key:(key_tonic,ml)}
}

pub fn analyze_progression(symbols:&[&str],key_tonic:i32,mode:&str,wrap:bool)->ProgressionAnalysis{
    let chords:Vec<Chord>=symbols.iter().map(|s|parse_roman(s,key_tonic,mode)).collect();
    let roots:Vec<i32>=chords.iter().map(|c|c.root).collect();let mut graph=TonalGraph::new();graph.build_from_progression(&roots);
    let _=compute_holonomy(&roots,wrap);
    let mut mods=Vec::new();for(i,c)in chords.iter().enumerate(){if c.is_secondary_dominant{mods.push((i,format!("Sec dom {}",c.function)))}else if !c.is_diatonic&&i>0{mods.push((i,format!("Non-diatonic {}",c.function)))}}
    let diat=chords.iter().filter(|c|c.is_diatonic).count();let stab=if chords.is_empty(){0.0}else{(diat as f64/chords.len()as f64).clamp(0.0,1.0)};
    ProgressionAnalysis{chords,graph,modulations:mods,modal_interchanges:vec![],stability_score:stab}
}

fn parse_acc(s:&str)->(String,&str){
    if s.starts_with("bb"){("bb".into(),&s[2..])}
    else if s.starts_with("##"){("##".into(),&s[2..])}
    else if s.starts_with('b')&&s.len()>1&&s[1..].chars().next().map(|c|c.is_uppercase()).unwrap_or(false){("b".into(),&s[1..])}
    else if s.starts_with('#'){("#".into(),&s[1..])}
    else{(String::new(),s)}
}
fn num_to_degree(s:&str)->usize{match s{"I"|"i"=>0,"II"|"ii"=>1,"III"|"iii"=>2,"IV"|"iv"=>3,"V"|"v"=>4,"VI"|"vi"=>5,"VII"|"vii"=>6,_=>0}}
fn apply_acc(mut pc:i32,acc:&str)->i32{match acc{"bb"=>pc=(pc+10)%12,"##"=>pc=(pc+2)%12,"b"=>pc=(pc+11)%12,"#"=>pc=(pc+1)%12,_=>{}};pc}

#[cfg(test)]mod tests{use super::*;#[test]fn t_parse_i(){let c=parse_roman("I",0,"major");assert_eq!(c.root,0);assert!(c.is_diatonic)}#[test]fn t_parse_v(){assert_eq!(parse_roman("V",0,"major").root,7)}#[test]fn t_analyze(){let a=analyze_progression(&["I","IV","V","I"],0,"major",true);assert_eq!(a.chords.len(),4)}}
